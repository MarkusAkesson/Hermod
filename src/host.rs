use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

static HOST_DIR: &str = "~/.hermod/known_hosts/";

pub struct Host {
    pub hostname: String,
    pub id_token: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub server_key: Vec<u8>,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.hostname)?;
        write!(f, "\tPublicKey: {}\n", base64::encode(&self.public_key))?;
        write!(f, "\tServerKey: {}\n", base64::encode(&self.private_key))?;
        write!(f, "\tIdToken: {}\n", base64::encode(&self.id_token))
    }
}

impl Host {
    pub fn new() -> Self {
        Host {
            hostname: String::new(),
            id_token: String::new(),
            public_key: Vec::new(),
            private_key: Vec::new(),
            server_key: Vec::new(),
        }
    }

    pub fn set_hostname(mut self, hostname: &str) -> Self {
        self.hostname.push_str(hostname);
        self
    }

    pub fn set_id_token(mut self, id: &str) -> Self {
        self.id_token.push_str(id);
        self
    }

    pub fn set_public_key(mut self, key: &[u8]) -> Self {
        self.public_key.extend(key);
        self
    }

    pub fn set_private_key(mut self, key: &[u8]) -> Self {
        self.private_key.extend(key);
        self
    }

    pub fn set_server_key(mut self, key: &[u8]) -> Self {
        self.server_key.extend(key);
        self
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    pub fn private_key(&self) -> &[u8] {
        &self.private_key
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn server_key(&self) -> &[u8] {
        &self.server_key
    }

    pub fn write_to_file(&self) -> io::Result<()> {
        unimplemented!();
    }
}

pub fn load_host(hostname: &str) -> Result<Host, &'static str> {
    let mut path = PathBuf::new();
    path.push(HOST_DIR);
    path.push(hostname);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut host = Host::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Split at whitespace and compare key: instead?
        let parts: Vec<&str> = line.split(":").collect();
        host = match parts[0] {
            "PublicKey" => host.set_public_key(&base64::decode(parts[1]).unwrap()),
            "PrivateKey" => host.set_private_key(&base64::decode(parts[1]).unwrap()),
            "Hostname" => host.set_hostname(parts[1]),
            "ID_Token" => host.set_id_token(parts[1]),
            "ServerKey" => host.set_server_key(&base64::decode(parts[1]).unwrap()),
            _ => host, // Unknown filed
        };
    }
    Ok(host)
}
