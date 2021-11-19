use crate::error::HermodError;
use crate::host::{self, Host};
use crate::identity::{Identity, KNOWN_CLIENTS};

use std::fmt;

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
            .ok_or_else(|| HermodError::UnknownIdentity)?;
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
