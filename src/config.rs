use crate::consts::SERVER_PRIVATE_KEY_FILE;
use crate::consts::SERVER_PUBLIC_KEY_FILE;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub trait Config {
    fn get_private_key(&self) -> &[u8];
    fn get_public_key(&self) -> &[u8];
}

pub struct ServerConfig {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

pub struct ClientConfig {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
    server_hostname: String,
    id_token: String,
    compression: bool,
}

impl Config for ServerConfig {
    fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }

    fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
}

impl Config for ClientConfig {
    fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }

    fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        let mut private_key = Vec::new();
        let mut public_key = Vec::new();

        let read_file = |buffer: &mut Vec<u8>, file_name: &str| -> io::Result<()> {
            let mut f = File::open(file_name)?;
            f.read_to_end(buffer)?;
            Ok(())
        };

        read_file(&mut private_key, SERVER_PRIVATE_KEY_FILE)
            .expect("Failed to read servers private key");
        read_file(&mut public_key, SERVER_PUBLIC_KEY_FILE)
            .expect("Failed tor ead servers public key");

        ServerConfig {
            public_key,
            private_key,
        }
    }
}
