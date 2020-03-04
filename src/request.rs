use crate::consts::*;
use crate::error::{HermodError, HermodErrorKind};
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;

use std::fmt;
use std::path::PathBuf;

use async_std::fs::File;
use async_std::io::{BufReader, BufWriter};
use async_std::prelude::*;

use indicatif::{ProgressBar, ProgressStyle};

use log::info;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub len: u64,
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
    pub fn new(source: &str, destination: &str, method: RequestMethod) -> Self {
        let mut destination = PathBuf::from(destination);
        let source = PathBuf::from(source);
        destination.push(source.file_name().unwrap());
        Request {
            source,
            destination,
            method,
        }
    }

    pub async fn respond(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        info!("Received new request: {}", self);
        match self.method {
            RequestMethod::Upload => self.download(endpoint, false).await,
            RequestMethod::Download => self.upload(endpoint, false).await,
        }
    }

    pub async fn exec(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        // TODO: move out of exec
        let enc_req = bincode::serialize(&self).unwrap();
        let msg = Message::new(MessageType::Request, &enc_req);
        endpoint.send(&msg).await?;
        info!("Sending request");
        match self.method {
            RequestMethod::Upload => self.upload(endpoint, true).await,
            RequestMethod::Download => self.download(endpoint, true).await,
        }
    }

    // read a file and send it to a task responsible for sending the msg to peer
    async fn upload(&self, endpoint: &mut Endpoint, is_client: bool) -> Result<(), HermodError> {
        let path = self.source.canonicalize()?;
        let file = File::open(&path).await?;
        let mut buf_reader = BufReader::new(file);

        let len = async_std::fs::metadata(&path).await?.len();
        let metadata = Metadata { len };
        let enc_metadata = bincode::serialize(&metadata).unwrap();
        let msg = Message::new(MessageType::Metadata, &enc_metadata);
        endpoint.send(&msg).await?;

        let filename = String::from(path.clone().file_name().unwrap().to_str().unwrap());
        let (tx, rx) = async_std::sync::channel(100);
        let pb = if is_client {
            Some(create_progress_bar(len, &filename))
        } else {
            None
        };
        // Spawns a task that reads a file and sends it to a receiver, responisble for sending the
        // messages to the endpoint/peer
        // TODO: Send Err on error instead of unwrapping
        async_std::task::spawn(async move {
            let mut read = 0u64;
            loop {
                let mut buffer = Vec::with_capacity(MSG_PAYLOAD_LEN);
                let n = buf_reader
                    .by_ref()
                    .take(MSG_PAYLOAD_LEN as u64)
                    .read_to_end(&mut buffer)
                    .await
                    .unwrap();

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
                pb.finish_with_message(format!("Uploaded: {:32} ", filename).as_str());
            }
        });

        while let Some(msg) = rx.recv().await {
            endpoint.send(&msg).await?;
        }

        Ok(())
    }

    async fn download(&self, endpoint: &mut Endpoint, is_client: bool) -> Result<(), HermodError> {
        let path = self.destination.clone();
        match path.parent() {
            Some(path) => {
                if !path.exists() {
                    async_std::fs::create_dir_all(&path).await?;
                }
            }
            None => (),
        };

        let msg = endpoint.recv().await?;
        if msg.get_type() == MessageType::Error {
            return Err(HermodError::new(HermodErrorKind::Other));
        }
        let metadata: Metadata = bincode::deserialize(msg.get_payload()).unwrap();
        let filename = String::from(path.clone().file_name().unwrap().to_str().unwrap());

        let pb = if is_client {
            Some(create_progress_bar(metadata.len, &filename))
        } else {
            None
        };

        let file = File::create(&path).await?;
        let mut buf_writer = BufWriter::new(file);

        let (tx, rx): (
            async_std::sync::Sender<Message>,
            async_std::sync::Receiver<Message>,
        ) = async_std::sync::channel(100);

        // Spawn a task that reads the file data to a file.
        async_std::task::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg.get_type() {
                    MessageType::Error => {
                        drop(buf_writer);
                        async_std::fs::remove_file(&path)
                            .await
                            .expect("Could not remove the destination file");

                        return; // Received error, log error message, Close Connection, Remove file
                    }
                    MessageType::Payload => {
                        let payload = msg.get_payload();

                        buf_writer
                            .write(payload)
                            .await
                            .expect("Failed to write payload to file");
                    }
                    MessageType::EOF => {
                        // EOF, flush buffer and return
                        // TODO: Log writing to file {} file.name
                        buf_writer
                            .flush()
                            .await
                            .expect("Failed to flush the file writer");
                        return;
                    }
                    _ => return, // log received unexpected message: {} type, Closing connection
                }
            }
        });

        // Recv messages until an Error or Close message has been received
        let mut received = 0u64;
        loop {
            let msg = endpoint.recv().await?;
            if msg.get_type() == MessageType::Error {
                if let Some(ref pb) = pb {
                    // TODO fix better error message
                    pb.finish_with_message(format!("Failed to upload: {:32} ", filename).as_str());
                }
                tx.send(msg).await;
                break;
            } else if msg.get_type() == MessageType::EOF {
                tx.send(msg).await;
                break;
            }

            if let Some(ref pb) = pb {
                received += msg.get_payload().len() as u64;
                pb.set_position(received);
            }

            tx.send(msg).await;
        }

        if let Some(ref pb) = pb {
            pb.finish_with_message(format!("Downloaded: {:32} ", filename).as_str());
        }

        Ok(())
    }
}

fn create_progress_bar(len: u64, file_name: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} [{elapsed_precise}] [{bar:43.cyan/blue}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})",
                )
                .progress_chars("#>-"),
        );
    pb.set_message(format!("Downloading: {:31}", file_name).as_str());
    pb
}
