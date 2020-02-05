use crate::message::Init;
use crate::noise::NoiseRole;
use crate::noise::NoiseSession;

use std::collections::HashMap;
use std::error::Error;

use async_std::net::TcpStream;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KNOWN_CLIENTS: HashMap<String, ClientEntry> = load_known_clients();
}

struct ClientEntry {
    id_token: String,
    public_key: Vec<u8>,
}

struct ServerEntry {
    ip: String,
    id_token: String,
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

pub struct Peer {
    id: String,
    stream: TcpStream,
    session: NoiseSession,
}

impl Peer {
    pub fn from_id_token(stream: TcpStream, msg: Init) -> Result<Self, Box<dyn Error>> {
        let client = KNOWN_CLIENTS.get(msg.id)?;
        let session = NoiseSession::new(client, SERVER_CONFIG, NoiseRole::Responder);

        Peer {
            id: client.id,
            stream,
            session,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn send(mesage: &message) {}
}
