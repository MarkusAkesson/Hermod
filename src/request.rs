use crate::config::ClientConfig;
use crate::message::{Message, MessageType};

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

    pub async fn send(&self) {
        unimplemented!();
    }

    pub async fn exec(&self) {
        match self.method {
            RequestMethod::Upload => self.upload().await,
            RequestMethod::Download => self.download().await,
        }
    }

    // read a file and send it to a task responsible for sending the msg to peer
    async fn upload(&self) {
        let file = File::open(self.source).await.unwrap();
        let mut buf_reader = BufReader::new(file);
        let (tx, rx) = async_std::sync::channel(32);
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
                break;
            }
            let msg = Message::new(&buffer, MessageType::Payload);
            tx.send(msg);
        }
    }

    async fn download(&self) {
        let file = File::create(self.source).await.unwrap();
        let buf_reader = BufWriter::new(file);
    }
}
