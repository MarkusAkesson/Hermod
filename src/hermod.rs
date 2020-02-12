use crate::config::{ClientConfig, ServerConfig};
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;

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
    let message = Message::from_buffer(buffer[0], &buffer[1..]).unwrap();

    let peer = match message {
        Message::Init(ref msg) => Peer::new_client_peer(&msg.get_id()),
        _ => unimplemented!(), // Received unexpected message, close connection
    };

    let mut endpoint = Endpoint::server(stream, peer, &message).await;

    loop {
        let message = endpoint.recv().await;
        if message.get_type() == MessageType::Error {
            // TODO: Log error
            break;
        }

        let response = message.process();

        if let Some(res) = response {
            endpoint.send(&res).await;
        }
    }
    Ok(())
}
