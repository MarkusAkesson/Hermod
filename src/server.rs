use crate::config::ServerConfig;
use crate::message::parse_message_type;
use crate::message::MessageType;
use crate::noise::NoiseSession;
use crate::peer::Peer;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: ServerConfig) {
        Server { config }
    }

    pub fn run_server(&self) {
        task::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:8080").await?;
            println!("Listening on {}", listener.local_addr()?);

            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next().await {
                let stream = stream?;
                task::spawn(async {
                    process(stream).await.unwrap();
                });
            }
            Ok(())
        });
    }
}

async fn process(stream: TcpStream) -> io::Result<()> {
    // log incomming packet from ip
    // try convert packet to HERMOD_MSG
    let mut buffer = Vec::new();
    let message = message::Init::new(&buffer)?;

    let peer = Peer::new(&stream, &message, NoiseRole::Responder).unwrap();
    let noise_session = NoiseSession::new(peer, serverConfig, NoiseRole::Responder)?;

    while let Some(packet) = peer.next().await {
        let message_type = parse_message_type(buffer[0])?;
        match message_type {
            MessageType::Request => unimplemented!(),
            MessageType::Payload => unimplemented!(),
            MessageType::Error => unimplemented!(),
            _ => unimplemented!(),
        }
    }
    Ok(())
}
