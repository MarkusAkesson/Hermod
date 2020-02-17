use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref KNOWN_CLIENTS: HashMap<String, Client> = Client::load_clients();
}

pub struct Client {
    pub id_token: String,
    pub client_key: Vec<u8>,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.id_token, base64::encode(&self.client_key))
    }
}

impl Client {
    pub fn new(id_token: String, client_key: Vec<u8>) -> Self {
        Client {
            id_token,
            client_key,
        }
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.client_key
    }

    pub fn get_id(&self) -> &str {
        &self.id_token
    }

    pub fn load_clients() -> HashMap<String, Client> {
        let mut path = PathBuf::new();
        path.push(dirs::home_dir().unwrap());
        path.push(".hermod/authorized_clients");

        let file = File::open(path).expect("Failed to open authorized_clients file");
        let lines = BufReader::new(file).lines();

        let mut clients = HashMap::new();

        for line in lines {
            let line = line.expect("Error reading authorized_clients file content");
            let line_content: Vec<&str> = line.split(":").collect();

            let id_token = line_content[0];
            let client_key = base64::decode(line_content[1].as_bytes()).unwrap();

            clients.insert(
                id_token.to_string(),
                Client {
                    id_token: id_token.to_string(),
                    client_key,
                },
            );
        }
        clients
    }
}

pub fn print_known_clients() {
    let num_clients = KNOWN_CLIENTS.len();

    if num_clients == 0 {
        println!("No known clients found.");
        return;
    }

    let mut writer = BufWriter::new(io::stdout());

    writer
        .write(format!("Found {} known client(s)\n", num_clients).as_ref())
        .unwrap();
    writer
        .write(format!("TOKEN PUBLIC_KEY\n").as_ref())
        .unwrap();
    KNOWN_CLIENTS.values().for_each(|v| {
        writer.write(format!("{}\n", v).as_ref()).unwrap();
    });
}
