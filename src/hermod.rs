use crate::config::{ClientConfig, ServerConfig};
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;

use std::str;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::sync::{channel, Receiver, Sender};
use async_std::task;

pub struct HermodClient {
    config: ClientConfig,
}

impl HermodClient {
    pub fn new(config: ClientConfig) -> Self {
        HermodClient { config }
    }

    pub fn execute(&self) {
        task::block_on(async {
            let mut stream = TcpStream::connect(self.config.get_hostname())
                .await
                .unwrap();
            let peer = Peer::new_server_peer(self.config.get_hostname());
            // Conduct noise handshake
            let mut endpoint = Endpoint::client(&mut stream, peer).await;
            // Execute the request
            let request = Request::new(&self.config);
            request.exec(&mut endpoint).await;
        });
    }
}

pub struct HermodServer {
    config: ServerConfig,
}

impl HermodServer {
    pub fn new(config: ServerConfig) -> HermodServer {
        HermodServer { config }
    }

    pub fn run_server(&self) {
        task::block_on(async {
            let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
            println!("Listening on {}", listener.local_addr().unwrap());

            let mut incoming = listener.incoming();
            while let Some(stream) = incoming.next().await {
                task::spawn(async {
                    let mut stream = stream.unwrap();
                    handle_connection(&mut stream).await.unwrap();
                });
            }
        });
    }
}

async fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    // log incomming packet from ip
    // try convert packet to HERMOD_MSG
    let mut buffer = Vec::new();
    let msg = Message::new(MessageType::from(buffer[0]), &buffer[1..]);

    let peer = match msg.get_type() {
        MessageType::Init => Peer::new_client_peer(&str::from_utf8(msg.get_payload()).unwrap()),
        _ => unimplemented!(), // Received unexpected message, log and drop connection
    };

    let mut endpoint = Endpoint::server(stream, peer, &msg).await;

    loop {
        let msg = endpoint.recv().await;
        if msg.get_type() == MessageType::Error {
            // TODO: Log error
            break;
        }

        match msg.get_type() {
            MessageType::Error => break, // Received error, log error message, Cloe Connection
            MessageType::Request => process_incomming_request(&msg, &mut endpoint).await,
            MessageType::Payload
            | MessageType::Unknown
            | MessageType::Init
            | MessageType::Response
            | MessageType::EOF => break, // log: Received message out of order {} type, Closing connection
        }
    }
    Ok(())
}

async fn process_incomming_request(msg: &Message, endpoint: &mut Endpoint) {
    let request: Request = bincode::deserialize(msg.get_payload()).unwrap();
    request.exec(endpoint).await;
}
