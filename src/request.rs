use crate::config::ClientConfig;
use crate::error::HermodError;
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;

use std::path::PathBuf;

use async_std::fs::File;
use async_std::io::{BufReader, BufWriter};
use async_std::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RequestMethod {
    Upload = 1,
    Download,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    source: PathBuf,
    destination: PathBuf,
    method: RequestMethod,
}

impl Request {
    pub fn new(config: &ClientConfig) -> Self {
        let mut destination = PathBuf::from(config.destination);
        let source = PathBuf::from(config.source);
        destination.push(source.file_name().unwrap());
        Request {
            source,
            destination,
            method: config.request,
        }
    }

    pub async fn respond(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        match self.method {
            RequestMethod::Upload => self.download(endpoint).await,
            RequestMethod::Download => self.upload(endpoint).await,
        }
    }

    pub async fn exec(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        // TODO: move out of exec
        let enc_req = bincode::serialize(&self).unwrap();
        let msg = Message::new(MessageType::Request, &enc_req);
        endpoint.send(&msg).await?;
        println!("Sending request");
        match self.method {
            RequestMethod::Upload => self.upload(endpoint).await,
            RequestMethod::Download => self.download(endpoint).await,
        }
    }

    // read a file and send it to a task responsible for sending the msg to peer
    async fn upload(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        let file = File::open(&self.source).await?;
        let mut buf_reader = BufReader::new(file);

        let (tx, rx) = async_std::sync::channel(100);

        // Spawn a task that reads a file and sends it to a receiver, responisble for sending the
        // messages to the endpoint/peer
        async_std::task::spawn(async move {
            loop {
                let mut buffer = Vec::with_capacity(1024);
                let n = buf_reader
                    .by_ref()
                    .take(1024)
                    .read_to_end(&mut buffer)
                    .await
                    .unwrap();
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
        });

        while let Some(msg) = rx.recv().await {
            endpoint.send(&msg).await?;
        }

        Ok(())
    }

    async fn download(&self, endpoint: &mut Endpoint) -> Result<(), HermodError> {
        let path = self.destination.clone();
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
                        async_std::fs::remove_file(path)
                            .await
                            .expect("Could not remove previously open file");
                        return; // Received error, log error message, Close Connection, Remove file
                    }
                    MessageType::Payload => {
                        let payload = msg.get_payload();
                        buf_writer.write(payload).await.unwrap();
                    }
                    MessageType::EOF => {
                        // EOF, flush buffer and return
                        // TODO: Log writing to file {} file.name
                        buf_writer.flush().await.unwrap();
                        return;
                    }
                    _ => return, // log received unexpected message: {} type, Closing connection
                }
            }
        });

        // Recv messages until an Error or Close message has been received
        loop {
            let msg = endpoint.recv().await?;
            println!("REQUEST: Received new message of type: {}", msg.get_type());
            if msg.get_type() == MessageType::Error || msg.get_type() == MessageType::EOF {
                tx.send(msg).await;
                break;
            }

            tx.send(msg).await;
        }

        Ok(())
    }
}
