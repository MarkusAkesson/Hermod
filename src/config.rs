use crate::consts::*;
use crate::host::Host;
use crate::request::RequestMethod;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

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
    host: &'builder Host,
    pub source: Option<&'builder [&'builder str]>,
    pub destination: Option<&'builder str>,
    pub request: Option<RequestMethod>,
}

pub struct ClientConfig<'builder> {
    host: &'builder Host,
    pub source: Vec<&'builder str>,
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
        &self.host.private_key()
    }

    fn get_public_key(&self) -> &[u8] {
        &self.host.public_key()
    }
}

impl<'builder> ClientConfigBuilder<'builder> {
    pub fn new(host: &'builder Host) -> Self {
        ClientConfigBuilder {
            host,
            source: None,
            destination: None,
            request: None,
        }
    }

    pub fn source(mut self, source: &'builder [&'builder str]) -> Self {
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

    pub fn build_config(&self) -> ClientConfig {
        let source = self.source.expect("No source file specified");
        let destination = self.destination.expect("No destination specified");
        let request = self.request.expect("No request method specified");

        ClientConfig::new(self.host, source, destination, request)
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        let mut public_key = Vec::with_capacity(SERVER_KEY_SIZE);
        let mut private_key = Vec::with_capacity(SERVER_KEY_SIZE);

        let read_key_file = |buffer: &mut Vec<u8>, file_name: &str| -> io::Result<()> {
            let mut path = PathBuf::new();
            path.push(dirs::home_dir().expect("Failed to get home directory"));
            path.push(HERMOD_BASE_DIR);
            path.push(file_name);
            let mut f = File::open(path)?;
            f.read_to_end(buffer)?;
            Ok(())
        };

        read_key_file(&mut public_key, SERVER_PUBLIC_KEY_FILE)
            .expect("Failed to read servers public key");
        read_key_file(&mut private_key, SERVER_PRIVATE_KEY_FILE)
            .expect("Failed to read servers private key");

        let public_key = base64::decode(&public_key).unwrap();
        let private_key = base64::decode(&private_key).unwrap();

        ServerConfig {
            public_key,
            private_key,
        }
    }
}

impl<'builder> ClientConfig<'builder> {
    pub fn new(
        host: &'builder Host,
        source: &'builder [&'builder str],
        destination: &'builder str,
        request: RequestMethod,
    ) -> Self {
        let source = source.to_vec();
        ClientConfig {
            host,
            source,
            destination,
            request,
        }
    }

    pub fn get_hostname(&self) -> &str {
        &self.host.hostname()
    }

    pub fn get_alias(&self) -> &str {
        &self.host.alias()
    }
}
