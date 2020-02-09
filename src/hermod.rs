use crate::config::{ClientConfig, ServerConfig};
use crate::message::Message;
use crate::peer::Endpoint;
use crate::peer::Peer;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

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
    let message = Message::from_buffer(&buffer).unwrap();

    let peer = match message {
        Message::Init(ref msg) => Peer::new_client_peer(&msg.get_id()),
        _ => unimplemented!(), // Received unexpected message, close connection
    };

    let mut endpoint = Endpoint::server(stream, peer, &message).await;

    while let Some(packet) = endpoint.next().await {
        let mut buffer = Vec::new();

        let message = Message::from_buffer(&buffer).unwrap();
        let response = match message {
            Message::Request(msg) => unimplemented!(),
            Message::Payload(msg) => unimplemented!(),
            Message::Error(msg) => unimplemented!(),
            _ => unimplemented!(),
        };

        endpoint.write(response.to_bytes()).await?;
    }
    Ok(())
}
