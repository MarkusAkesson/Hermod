use crate::config::{ClientConfig, SERVER_CONFIG};
use crate::error::{HermodError, HermodErrorKind};
use crate::host::{self, Host};
use crate::identity::{Identity, KNOWN_CLIENTS};
use crate::message::{Message, MessageType};
use crate::noise::NoiseStream;

use std::fmt;

use async_std::net::TcpStream;

pub enum Peer {
    Identity(Identity),
    Host(Host),
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_id())
    }
}

impl Peer {
    pub async fn new_server_peer(alias: &str) -> Result<Self, HermodError> {
        let host = host::load_host_async(alias).await?;
        Ok(Peer::Host(host))
    }

    pub async fn new_client_peer(id: &str) -> Result<Self, HermodError> {
        let clients = KNOWN_CLIENTS.lock().await;
        let client = clients
            .get(id)
            .ok_or_else(|| HermodError::new(HermodErrorKind::UnknownIdentity))?;
        let id_token = client.get_id().to_owned();
        let client_key = client.get_public_key().to_vec();

        Ok(Peer::Identity(Identity {
            id_token,
            client_key,
        }))
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

    pub async fn close(&mut self) -> Result<(), HermodError> {
        let msg = Message::new(MessageType::Close, &[]);
        self.stream.send(&msg).await
    }

    pub async fn send(&mut self, msg: &Message) -> Result<(), HermodError> {
        self.stream.send(msg).await
    }

    pub async fn recv(&mut self) -> Result<Message, HermodError> {
        self.stream.recv().await
    }
}
