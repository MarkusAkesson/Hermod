use crate::config::ClientConfig;
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;

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
pub struct Request<'a> {
    source: &'a str,
    destination: &'a str,
    method: RequestMethod,
}

impl<'a> Request<'a> {
    pub fn new(config: &'a ClientConfig) -> Self {
        Request {
            source: &config.source,
            destination: &config.destination,
            method: config.request,
        }
    }

    pub async fn respond(&self, endpoint: &mut Endpoint) {
        match self.method {
            RequestMethod::Upload => self.download(endpoint).await,
            RequestMethod::Download => self.upload(endpoint).await,
        }
    }

    pub async fn exec(&self, endpoint: &mut Endpoint) {
        match self.method {
            RequestMethod::Upload => self.upload(endpoint).await,
            RequestMethod::Download => self.download(endpoint).await,
        }
    }

    // read a file and send it to a task responsible for sending the msg to peer
    async fn upload(&self, endpoint: &mut Endpoint) {
        let file = File::open(self.source).await.unwrap();
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
                    // Send EOF message and break loop
                    // Handle EOF in send loop? when rx dropx send EOF?
                    break;
                }
                let msg = Message::new(MessageType::Payload, &buffer);
                tx.send(msg).await;
            }
        });
        while let Some(msg) = rx.recv().await {
            endpoint.send(&msg).await;
        }
    }

    async fn download(&self, endpoint: &mut Endpoint) {
        let file = File::create(self.destination).await.unwrap();
        let mut buf_writer = BufWriter::new(file);

        let (tx, rx): (
            async_std::sync::Sender<Message>,
            async_std::sync::Receiver<Message>,
        ) = async_std::sync::channel(100);

        // Spawn a task that reads the file data to a file.
        async_std::task::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg.get_type() {
                    MessageType::Error => return, // Received error, log error message, Close Connection, Remove file
                    MessageType::Payload => (),
                    MessageType::EOF => {
                        // EOF, flush buffer and return
                        // TODO: Log writing to file {} file.name
                        buf_writer.flush().await.unwrap();
                        return;
                    }
                    MessageType::Request
                    | MessageType::Unknown
                    | MessageType::Init
                    | MessageType::Response => return, // log Received message out of order{} type, Closing connection
                }
                let payload = msg.get_payload();
                buf_writer.write(payload).await.unwrap();
            }
            buf_writer.flush().await.unwrap();
        });

        // Recv messages until an Error message has been received or the tcp connection is dropped
        loop {
            let msg = endpoint.recv().await;
            if msg.get_type() == MessageType::Error {
                // TODO: Log error
                break;
            }

            tx.send(msg).await;
        }
    }
}