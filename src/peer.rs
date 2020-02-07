use crate::client::{Client, KNOWN_CLIENTS};
use crate::config::{CLIENT_CONFIG, SERVER_CONFIG};
use crate::message::{Message, MessageType};
use crate::noise::NoiseRole;
use crate::noise::NoiseStream;
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
    stream: NoiseStream,
}

impl Endpoint {
    pub async fn client(stream: &mut TcpStream, peer: Peer) -> Self {
        let stream = NoiseStream::new_initiator(&peer, &*CLIENT_CONFIG, stream)
            .await
            .unwrap();

        Endpoint { peer, stream }
    }

    pub async fn server(stream: &mut TcpStream, peer: Peer, msg: &Message) -> Self {
        let stream = NoiseStream::new_responder(&peer, &*SERVER_CONFIG, stream, msg)
            .await
            .unwrap();

        Endpoint { peer, stream }
    }
}
