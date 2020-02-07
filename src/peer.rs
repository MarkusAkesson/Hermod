use crate::client::{Client, KNOWN_CLIENTS};
use crate::config::{CLIENT_CONFIG, SERVER_CONFIG};
use crate::message::Message;
use crate::noise::NoiseRole;
use crate::noise::NoiseSession;
use crate::server::Server;

use std::error::Error;
use std::pin::Pin;

use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::stream::Stream;
use async_std::task::{Context, Poll};

pub enum Peer {
    Client(Client),
    Server(Server),
}

impl Peer {
    pub fn new_server_peer(hostname: &str) -> Self {
        unimplemented!()
    }

    pub fn new_client_peer(id_token: &str) -> Self {
        let client = KNOWN_CLIENTS.get(id_token).unwrap();

        Peer::Client(Client {
            id_token: client.get_id().to_owned(),
            client_key: client.get_public_key().to_vec(),
        })
    }

    pub fn get_id(&self) -> &str {
        match self {
            Peer::Client(client) => &client.id_token,
            Peer::Server(server) => &server.hostname,
        }
    }

    pub fn get_public_key(&self) -> &[u8] {
        match self {
            Peer::Client(client) => &client.client_key,
            Peer::Server(server) => &server.public_key,
        }
    }
}

impl Stream for Endpoint {
    type Item = u8;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!();
    }
}

pub struct Endpoint {
    peer: Peer,
    stream: TcpStream,
    session: NoiseSession,
}

impl Endpoint {
    pub fn client(stream: TcpStream, peer: Peer) -> Self {
        let session = NoiseSession::new(&peer, &*CLIENT_CONFIG, NoiseRole::Initiator).unwrap();

        Endpoint {
            peer,
            stream,
            session,
        }
    }

    pub fn server(stream: TcpStream, peer: Peer) -> Self {
        let session = NoiseSession::new(&peer, &*SERVER_CONFIG, NoiseRole::Responder).unwrap();

        Endpoint {
            peer,
            stream,
            session,
        }
    }

    pub async fn send(&mut self, message: &Message) {
        let len = message.len();
        let mut payload = Vec::with_capacity(len);
        self.session
            .write_message(message.as_bytes(), &mut payload)
            .unwrap();
        self.stream.write_all(&payload).await.unwrap();
    }
}
