use crate::config::ClientConfig;
use crate::message::{Message, MessageType};
use crate::noise::NoiseSession;
use crate::peer::Peer;
use crate::request::Request;

use async_std::net::TcpStream;

pub struct Client<'hc> {
    config: ClientConfig<'hc>,
}

impl<'hc> Client<'hc> {
    pub fn new(config: ClientConfig<'hc>) -> Self {
        Client { config }
    }

    pub fn execute(&self) {
        async_std::task::block_on(async {
            let mut stream = match TcpStream::connect(self.config.get_hostname()).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("Failed to connect to server: {}", e);
                    return;
                }
            };
            let peer = match Peer::new_server_peer(self.config.get_alias()).await {
                Ok(peer) => peer,
                Err(_) => {
                    log::error!(
                        "Cound not find a server with that alias ({}). Aborting...",
                        self.config.get_alias()
                    );
                    return;
                }
            };

            // Conduct noise handshake
            // TODO: Better error message
            let mut session = NoiseSession::as_initiator(peer, &self.config, &mut stream)
                .await
                .expect("Failed to create a secure tunnel to the server");
            match Request::from(&self.config) {
                Ok(requests) => {
                    // Execute the requests
                    match Request::exec_all(&mut session, &requests).await {
                        Ok(_) => (),
                        Err(e) => {
                            // Close connection on first error
                            // Try to send the other requests on error?
                            log::error!("Failed to execute the request: {}", e);
                        }
                    }
                }
                Err(e) => log::error!("{}", e),
            };

            if let Err(e) = session.send(&Message::new(MessageType::Close, &[])).await {
                log::error!("{}", e);
            }
        });
    }
}
