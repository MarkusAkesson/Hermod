use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader, BufWriter};
use std::path::PathBuf;

static HOST_DIR: &str = ".hermod/known_hosts";

pub struct Host {
    pub alias: String,
    pub hostname: String,
    pub id_token: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub server_key: Vec<u8>,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.alias)?;
        write!(f, "\tPublicKey: {}\n", base64::encode(&self.public_key))?;
        write!(f, "\tIdToken: {}\n", base64::encode(&self.id_token))
    }
}

impl Host {
    pub fn with_alias(alias: &str) -> Self {
        Host {
            alias: String::from(alias),
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
        let mut path = PathBuf::new();
        path.push(dirs::home_dir().unwrap());
        path.push(HOST_DIR);
        path.push(&self.alias);

        println!("Path: {:?}", path);

        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(format!("Hostname: {}", &self.hostname).as_bytes())?;
        writer.write(format!("Publickey: {}", base64::encode(&self.public_key)).as_bytes())?;
        writer.write(format!("PrivateKey: {}", base64::encode(&self.private_key)).as_bytes())?;
        writer.write(format!("IdToken: {}", &self.id_token).as_bytes())?;
        writer.write(format!("ServerKey: {}", base64::encode(&self.server_key)).as_bytes())?;
        unimplemented!();
    }
}

pub fn load_host(alias: &str) -> Result<Host, &'static str> {
    let mut path = PathBuf::new();
    path.push(dirs::home_dir().unwrap());
    path.push(HOST_DIR);
    path.push(alias);

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut host = Host::with_alias(alias);

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
