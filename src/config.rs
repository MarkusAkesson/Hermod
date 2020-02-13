use crate::consts::SERVER_PRIVATE_KEY_FILE;
use crate::consts::SERVER_PUBLIC_KEY_FILE;
use crate::request::{Request, RequestMethod};

use std::fs::File;
use std::io;
use std::io::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = ServerConfig::new();
}

pub trait Config<'cfg>: Send + Sync {
    fn get_private_key(&self) -> &[u8];
    fn get_public_key(&self) -> &[u8];
}

#[derive(Clone)]
pub struct ServerConfig {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

pub struct ClientConfigBuilder<'builder> {
    public_key: &'builder [u8],
    private_key: &'builder [u8],
    hostname: &'builder str,
    id_token: &'builder str,
    compression: Option<bool>,
    pub source: Option<&'builder str>,
    pub destination: Option<&'builder str>,
    pub request: Option<RequestMethod>,
}

pub struct ClientConfig<'builder> {
    public_key: &'builder [u8],
    private_key: &'builder [u8],
    hostname: &'builder str,
    id_token: &'builder str,
    compression: bool,
    pub source: &'builder str,
    pub destination: &'builder str,
    pub request: RequestMethod,
}

impl Config<'_> for ServerConfig {
    fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }

    fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
}

impl<'builder> Config<'builder> for ClientConfig<'builder> {
    fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }

    fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
}

impl<'builder> ClientConfigBuilder<'builder> {
    pub fn new(
        public_key: &'builder [u8],
        private_key: &'builder [u8],
        hostname: &'builder str,
        id_token: &'builder str,
    ) -> Self {
        ClientConfigBuilder {
            public_key,
            private_key,
            hostname,
            id_token,
            compression: None,
            source: None,
            destination: None,
            request: None,
        }
    }

    pub fn compresison(mut self, compression: bool) -> Self {
        self.compression = Some(compression);
        self
    }

    pub fn source(mut self, source: &'builder str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn destination(mut self, destination: &'builder str) -> Self {
        self.destination = Some(destination);
        self
    }

    pub fn request(mut self, request: RequestMethod) -> Self {
        self.request = Some(request);
        self
    }

    pub fn build(&self) -> ClientConfig {
        let compression = self.compression.unwrap_or(false);
        let source = self.source.expect("No source file specified");
        let destination = self.destination.expect("No destination specified");
        let request = self.request.expect("No request method specified");

        ClientConfig::new(
            self.public_key,
            self.private_key,
            self.hostname,
            self.id_token,
            compression,
            source,
            destination,
            request,
        )
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
            .expect("Failed to read servers public key");

        ServerConfig {
            public_key,
            private_key,
        }
    }
}

impl<'builder> ClientConfig<'builder> {
    pub fn new(
        public_key: &'builder [u8],
        private_key: &'builder [u8],
        hostname: &'builder str,
        id_token: &'builder str,
        compression: bool,
        source: &'builder str,
        destination: &'builder str,
        request: RequestMethod,
    ) -> Self {
        ClientConfig {
            public_key,
            private_key,
            hostname,
            id_token,
            compression,
            source,
            destination,
            request,
        }
    }

    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }
}