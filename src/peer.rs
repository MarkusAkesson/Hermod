use crate::config::{ClientConfig, SERVER_CONFIG};
use crate::error::HermodError;
use crate::host::{self, Host};
use crate::identity::{Identity, KNOWN_CLIENTS};
use crate::message::Message;
use crate::noise::NoiseStream;

use async_std::net::TcpStream;

pub enum Peer {
    Identity(Identity),
    Host(Host),
}

impl Peer {
    pub async fn new_server_peer(alias: &str) -> Result<Self, HermodError> {
        let host = host::load_host_async(alias).await?;
        Ok(Peer::Host(host))
    }

    pub fn new_client_peer(id_token: &str) -> Self {
        println!("{}", id_token);
        let id = KNOWN_CLIENTS.get(id_token).unwrap();

        Peer::Identity(Identity {
            id_token: id.get_id().to_owned(),
            client_key: id.get_public_key().to_vec(),
        })
    }

    pub fn get_id(&self) -> &str {
        match self {
            Peer::Identity(id) => &id.id_token,
            Peer::Host(host) => &host.id_token,
        }
    }

    pub fn get_public_key(&self) -> &[u8] {
        match self {
            Peer::Identity(id) => &id.client_key,
            Peer::Host(host) => &host.server_key,
        }
    }
}

pub struct Endpoint {
    peer: Peer,
    stream: NoiseStream,
}

impl<'e> Endpoint {
    pub async fn client(
        stream: &mut TcpStream,
        peer: Peer,
        cfg: &ClientConfig<'e>,
    ) -> Result<Self, HermodError> {
        let stream = NoiseStream::new_initiator(&peer, cfg, stream).await?;

        Ok(Endpoint { peer, stream })
    }

    pub async fn server(
        stream: &mut TcpStream,
        peer: Peer,
        msg: &Message,
    ) -> Result<Self, HermodError> {
        let stream = NoiseStream::new_responder(&peer, &*SERVER_CONFIG, stream, msg).await?;

        Ok(Endpoint { peer, stream })
    }

    pub fn get_stream(&self) -> &TcpStream {
        self.stream.get_stream()
    }

    pub fn get_peer(&self) -> &Peer {
        &self.peer
    }

    pub async fn send(&mut self, msg: &Message) -> Result<(), HermodError> {
        self.stream.send(msg).await
    }

    pub async fn recv(&mut self) -> Result<Message, HermodError> {
        self.stream.recv().await
    }
}
