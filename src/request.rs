use crate::config::ClientConfig;
use crate::consts::*;
use crate::error::{HermodError, HermodErrorKind};
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;

use std::fmt;
use std::path::PathBuf;

use async_std::fs::{self, DirEntry, File};
use async_std::io::{BufReader, BufWriter};
use async_std::path::Path;
use async_std::prelude::*;
use async_std::sync::{Receiver, Sender};

use futures::future::{FutureExt, LocalBoxFuture};
use futures::{stream, Stream, StreamExt};

use indicatif::{ProgressBar, ProgressStyle};

use log::{debug, error, info};

use serde::{Deserialize, Serialize};

use walkdir::WalkDir;

pub struct PathList {
    paths: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub len: u64,
    pub file_name: String,
    pub dir: bool,
}

impl Metadata {
    pub async fn from_path(path: &PathBuf) -> Result<Self, HermodError> {
        let file_name = String::from(path.clone().file_name().unwrap().to_str().unwrap());
        let len = async_std::fs::metadata(&path).await?.len();
        let dir = async_std::fs::metadata(&path).await?.is_dir();
        Ok(Metadata {
            len,
            dir,
            file_name,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RequestMethod {
    Upload = 1,
    Download,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestMethod::Upload => write!(f, "Upload"),
            RequestMethod::Download => write!(f, "Download"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    source: PathBuf,
    destination: PathBuf,
    method: RequestMethod,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.method {
            RequestMethod::Upload => write!(
                f,
                "{} {:?} to {}",
                self.method,
                self.source.file_name().unwrap(),
                self.destination.as_path().display(),
            ),
            RequestMethod::Download => write!(
                f,
                "{} {} from {}",
                self.method,
                self.source.as_path().display(),
                self.destination.as_path().display(),
            ),
        }
    }
}

impl Request {
    pub fn file(
        source: &str,
        destination: &str,
        method: RequestMethod,
    ) -> Result<Self, HermodError> {
        let mut destination = PathBuf::from(destination);
        let source = PathBuf::from(source);
        destination.push(source.file_name().unwrap());

        if method == RequestMethod::Upload {
            source
                .canonicalize()
                .map_err(|e| HermodError::new(HermodErrorKind::FileNotFound(e)))?;
        }

        Ok(Request {
            source,
            destination,
            method,
        })
    }

    pub fn dir(
        source: &str,
        destination: &str,
        method: RequestMethod,
    ) -> Result<Vec<Request>, HermodError> {
        let mut requests = Vec::new();
        for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_dir() {
                requests.push(Request::file(&path.to_string_lossy(), destination, method)?);
            }
        }
        Ok(requests)
    }

    // FIXME Ugly hack for creating multiple requests from one config
    // Should probably be one constructor for upload and one for download
    pub fn from(config: &ClientConfig<'_>) -> Result<Vec<Request>, HermodError> {
        let destination = config.destination;
        let method = config.request;
        let mut requests = Vec::new();
        for path in &config.source {
            let source = PathBuf::from(path);
            if method == RequestMethod::Upload && source.is_dir() {
                requests.append(&mut Request::dir(path, destination, method)?);
            } else {
                requests.push(Request::file(path, destination, method)?);
            }
        }
        Ok(requests)
    }

    pub async fn respond(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        info!(
            "Received new request from {}: {}",
            endpoint.get_peer(),
            self
        );
        match self.method {
            RequestMethod::Upload => self.download_server(endpoint).await,
            RequestMethod::Download => self.upload_server(endpoint).await,
        }
    }

    pub async fn exec_all(
        endpoint: &mut Endpoint,
        requests: &[Request],
    ) -> Result<(), HermodError> {
        for request in requests {
            request.exec(endpoint).await?;
        }

        Ok(())
    }

    pub async fn exec(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        // TODO: move out of exec
        let enc_req = bincode::serialize(&self).unwrap();
        let msg = Message::new(MessageType::Request, &enc_req);
        endpoint.send(&msg).await?;
        info!("Sending request");
        match self.method {
            RequestMethod::Upload => self.upload_client(endpoint).await,
            RequestMethod::Download => self.download_client(endpoint).await,
        }
    }

    async fn upload_server(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        let path = self.source.canonicalize()?;
        let (tx, rx) = async_std::sync::channel(100);

        if path.as_path().is_dir() {
            let paths = read_dir(&path);
            paths
                .for_each(|entry| async {
                    match entry {
                        Ok(path) => debug!("{}", path.path().display()),
                        Err(e) => error!("Encountered an error: {}", e),
                    };
                })
                .await;
        } else {
            // Move to own function
            let file = File::open(&path).await?;
            let buf_reader = BufReader::new(file);
            let metadata = Metadata::from_path(&path).await?;
            let enc_metadata = bincode::serialize(&metadata).unwrap();
            let msg = Message::new(MessageType::Metadata, &enc_metadata);
            endpoint.send(&msg).await?;

            // Spawns a task that reads a file and sends it to a receiver, responisble for sending the
            // messages to the endpoint/peer
            // TODO: Send Err on error instead of unwrapping
            async_std::task::spawn(
                async move { read_file(buf_reader, tx, &metadata, false).await },
            );
        }
        while let Some(msg) = rx.recv().await {
            endpoint.send(&msg).await?;
        }

        Ok(())
    }

    async fn upload_client(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        let path = self.source.canonicalize()?;
        let file = File::open(&path).await?;
        let buf_reader = BufReader::new(file);

        let metadata = Metadata::from_path(&path).await?;

        let (tx, rx) = async_std::sync::channel(100);
        // Spawns a task that reads a file and sends it to a receiver, responisble for sending the
        // messages to the endpoint/peer
        // TODO: Send Err on error instead of unwrapping
        async_std::task::spawn(async move { read_file(buf_reader, tx, &metadata, true).await });

        while let Some(msg) = rx.recv().await {
            endpoint.send(&msg).await?;
        }

        Ok(())
    }

    async fn download_server(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        let path = async_std::path::PathBuf::from(&self.destination);
        match path.parent() {
            Some(path) => {
                if !path.exists().await {
                    async_std::fs::create_dir_all(&path).await?;
                }
            }
            None => (),
        };

        let file = File::create(&path).await?;
        let buf_writer = BufWriter::new(file);

        let (tx, rx): (Sender<Message>, Receiver<Message>) = async_std::sync::channel(100);

        // Spawn a task that write the incoming payload to disk
        async_std::task::spawn(async move { write_file(buf_writer, rx, path.as_path()).await });

        // Recv messages until an Error or Close message has been received
        loop {
            let msg = endpoint.recv().await?;
            if msg.get_type() == MessageType::Error {
                tx.send(msg).await;
                break;
            } else if msg.get_type() == MessageType::EOF {
                tx.send(msg).await;
                break;
            }

            tx.send(msg).await;
        }

        Ok(())
    }

    async fn download_client(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        let path = async_std::path::PathBuf::from(&self.destination);
        match path.parent() {
            Some(path) => {
                if !path.exists().await {
                    async_std::fs::create_dir_all(&path).await?;
                }
            }
            None => (),
        };
        // Recv metadata about the file that is going to be transmitted
        let msg = endpoint.recv().await?;

        if msg.get_type() == MessageType::Error {
            return Err(HermodError::new(HermodErrorKind::Other));
        }

        let metadata: Metadata = bincode::deserialize(msg.get_payload()).unwrap();

        if metadata.dir {
            // download dir
            println!("Downloading directory: {}.", path.as_path().display());
            self.download_dir(endpoint, path, &metadata).await
        } else {
            self.download_file(endpoint, path, &metadata).await
        }
    }

    async fn download_dir(
        &self,
        endpoint: &mut Endpoint,
        path: async_std::path::PathBuf,
        metadata: &Metadata,
    ) -> Result<(), HermodError> {
        unimplemented!()
    }

    async fn download_file(
        &self,
        endpoint: &mut Endpoint,
        path: async_std::path::PathBuf,
        metadata: &Metadata,
    ) -> Result<(), HermodError> {
        // download file
        // Should probably be a function

        let pb = create_progress_bar(&metadata);

        let file = File::create(&path).await?;
        let buf_writer = BufWriter::new(file);

        let (tx, rx): (Sender<Message>, Receiver<Message>) = async_std::sync::channel(100);

        // Spawn a task that write the incoming payload to disk
        async_std::task::spawn(async move { write_file(buf_writer, rx, path.as_path()).await });

        // Recv messages until an Error or Close message has been received
        let mut received = 0u64;
        loop {
            let msg = endpoint.recv().await?;
            if msg.get_type() == MessageType::Error {
                // TODO fix better error message
                pb.finish_with_message(
                    format!("Failed to download: {:32} ", metadata.file_name).as_str(),
                );
                tx.send(msg).await;
                break;
            } else if msg.get_type() == MessageType::EOF {
                tx.send(msg).await;
                break;
            }

            received += msg.get_payload().len() as u64;
            pb.set_position(received);

            tx.send(msg).await;
        }

        pb.finish_with_message(format!("Downloaded: {:32} ", metadata.file_name).as_str());
        Ok(())
    }
}

fn read_dir(
    path: impl Into<async_std::path::PathBuf>,
) -> impl Stream<Item = Result<DirEntry, HermodError>> {
    async fn read_dir_internal(
        path: async_std::path::PathBuf,
        to_visit: &mut Vec<async_std::path::PathBuf>,
    ) -> Result<Vec<DirEntry>, HermodError> {
        let mut dir = fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(entry) = dir.next().await {
            let entry = entry?;
            if entry.metadata().await?.is_dir() {
                to_visit.push(entry.path());
            } else {
                files.push(entry);
            }
        }
        Ok(files)
    }

    stream::unfold(vec![path.into()], |mut to_visit| async {
        let path = to_visit.pop()?;
        let file_stream = match read_dir_internal(path, &mut to_visit).await {
            Ok(files) => stream::iter(files).map(Ok).left_stream(),
            Err(e) => stream::once(async { Err(e) }).right_stream(),
        };
        Some((file_stream, to_visit))
    })
    .flatten()
}

async fn read_file(
    mut reader: BufReader<File>,
    tx: Sender<Message>,
    metadata: &Metadata,
    is_client: bool,
) {
    let pb = if is_client {
        Some(create_progress_bar(metadata))
    } else {
        None
    };

    let mut read = 0u64;
    loop {
        let mut buffer = Vec::with_capacity(MSG_PAYLOAD_LEN);
        let n = reader
            .by_ref()
            .take(MSG_PAYLOAD_LEN as u64)
            .read_to_end(&mut buffer)
            .await
            .expect("Failed to read from the file");

        if let Some(ref pb) = pb {
            read += n as u64;
            pb.set_position(read);
        }

        if n == 0 {
            // EOF reached
            // Send EOF to peer
            let msg = Message::new(MessageType::EOF, &[]);
            tx.send(msg).await;
            break;
        }
        let msg = Message::new(MessageType::Payload, &buffer);
        tx.send(msg).await;
    }

    // TODO handle error
    if let Some(ref pb) = pb {
        pb.finish_with_message(format!("Uploaded: {:32} ", &metadata.file_name).as_str());
    }
}

async fn write_file(mut writer: BufWriter<File>, rx: Receiver<Message>, path: &Path) {
    while let Some(msg) = rx.recv().await {
        match msg.get_type() {
            MessageType::Error => {
                drop(writer);
                async_std::fs::remove_file(&path)
                    .await
                    .expect("Could not remove the destination file");

                return; // Received error, log error message, Close Connection, Remove file
            }
            MessageType::Payload => {
                let payload = msg.get_payload();

                writer
                    .write(payload)
                    .await
                    .expect("Failed to write payload to file");
            }
            MessageType::EOF => {
                // EOF, flush buffer and return
                // TODO: Log writing to file {} file.name
                writer
                    .flush()
                    .await
                    .expect("Failed to flush the file writer");
                return;
            }
            _ => return, // log received unexpected message: {} type, Closing connection
        }
    }
}

fn create_progress_bar(metadata: &Metadata) -> ProgressBar {
    let pb = ProgressBar::new(metadata.len);
    pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} [{elapsed_precise}] [{bar:43.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})",
                )
                .progress_chars("#>-"),
        );
    pb.set_message(format!("Downloading: {:31}", metadata.file_name).as_str());
    pb
}
