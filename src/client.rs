use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

lazy_static! {
    pub static ref KNOWN_CLIENTS: HashMap<String, Client> =
        Client::load_clients(Path::new("~/.hermod/authorized_clients"));
}

pub struct Client {
    pub id_token: String,
    pub client_key: Vec<u8>,
}

impl Client {
    pub fn new(id_token: String, client_key: Vec<u8>) -> Self {
        Client {
            id_token,
            client_key,
        }
    }

    pub fn load_clients(path: &Path) -> HashMap<String, Client> {
        let file = File::open(path).expect("Failed to open authorized_clients file");
        let lines = BufReader::new(file).lines();

        let clients = HashMap::new();

        for line in lines {
            let line_content: Vec<String> = line
                .expect("Error reading authorized_clients file content")
                .split(":")
                .collect();

            let id_token = line_content[0];
            let client_key = Vec::from(line_content[1]);

            clients.insert(
                id_token,
                Client {
                    id_token,
                    client_key,
                },
            );
        }
        clients
    }
}
