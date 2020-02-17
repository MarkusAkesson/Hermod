use crate::config::ClientConfig;
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;

use async_std::net::TcpStream;
use async_std::task;

pub struct HermodClient<'hc> {
    config: ClientConfig<'hc>,
}

impl<'hc> HermodClient<'hc> {
    pub fn new(config: ClientConfig<'hc>) -> Self {
        HermodClient { config }
    }

    pub fn execute(&self) {
        async_std::task::block_on(async {
            let mut stream = TcpStream::connect(self.config.get_hostname())
                .await
                .unwrap();
            let peer = Peer::new_server_peer(self.config.get_alias());
            // Conduct noise handshake
            let mut endpoint = Endpoint::client(&mut stream, peer, &self.config).await;
            // Execute the request
            let request = Request::new(&self.config);
            request.exec(&mut endpoint).await;
        });
    }
}
