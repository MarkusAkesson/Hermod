use crate::config::ClientConfig;
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
            match Request::from(&self.config) {
                Ok(requests) => {
                    // Execute the requests
                    match Request::exec_all(&mut endpoint, &requests).await {
                        Ok(_) => (),
                        Err(e) => {
                            // Close connection on first error
                            // Try to send the other requests on error?
                            eprintln!("Failed to execute the request: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("{}", e),
            };
        });
    }
}
