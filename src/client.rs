use crate::config::ClientConfig;
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;

use async_std::net::TcpStream;

pub struct HermodClient<'hc> {
    config: ClientConfig<'hc>,
}

impl<'hc> HermodClient<'hc> {
    pub fn new(config: ClientConfig<'hc>) -> Self {
        HermodClient { config }
    }

    pub fn execute(&self) {
        async_std::task::block_on(async {
            let mut stream = match TcpStream::connect(self.config.get_hostname()).await {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Failed to connect to server: {}", e);
                    return;
                }
            };
            let peer = match Peer::new_server_peer(self.config.get_alias()).await {
                Ok(peer) => peer,
                Err(_) => {
                    println!(
                        "Cound not find a server with that alias ({}). Aborting...",
                        self.config.get_alias()
                    );
                    return;
                }
            };
            // Conduct noise handshake
            // TODO: Better error message
            let mut endpoint = Endpoint::client(&mut stream, peer, &self.config)
                .await
                .unwrap();
            // Execute the request
            for src in &self.config.source {
                let request = Request::new(src, self.config.destination, self.config.request);
                match request.exec(&mut endpoint).await {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to execute the request: {}", e);
                        return;
                    }
                };
            }
            let msg = Message::new(MessageType::Close, &[]);
            match endpoint.send(&msg).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to close the connection: {}", e);
                    return;
                }
            };
        });
    }
}
