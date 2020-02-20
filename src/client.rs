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
                    println!("{}", e);
                    return;
                }
            };
            let peer = Peer::new_server_peer(self.config.get_alias());
            // Conduct noise handshake
            let mut endpoint = Endpoint::client(&mut stream, peer, &self.config)
                .await
                .unwrap();
            // Execute the request
            let request = Request::new(&self.config);
            request.exec(&mut endpoint).await;
            let msg = Message::new(MessageType::Close, &[]);
            endpoint.send(&msg).await;
        });
    }
}
