use crate::consts::*;
use crate::error::HermodError;

use std::fmt;

use std::io::prelude::*;

use async_std::fs::File;
use async_std::io::{self, BufReader};
use async_std::path::PathBuf;
use async_std::prelude::*;

static HOST_DIR: &str = "known_hosts";

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
        writeln!(f, "{}", self.alias)?;
        writeln!(f, "\tPublicKey: {}", base64::encode(&self.public_key))?;
        writeln!(f, "\tIdToken: {}", base64::encode(&self.id_token))
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

    pub fn alias(&self) -> &str {
        &self.alias
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

    // Only used from sync blocks
    pub fn write_to_file(&self) -> io::Result<()> {
        let mut path = PathBuf::new();
        path.push(dirs::home_dir().unwrap());
        path.push(HERMOD_BASE_DIR);
        path.push(HOST_DIR);

        std::fs::create_dir_all(&path).expect("Failed to crate known_hosts directory");

        path.push(&self.alias);

        let file = std::fs::File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(format!("Hostname: {}:{}\n", &self.hostname, HERMOD_PORT).as_bytes())?;
        writer
            .write_all(format!("PublicKey: {}\n", base64::encode(&self.public_key)).as_bytes())?;
        writer
            .write_all(format!("PrivateKey: {}\n", base64::encode(&self.private_key)).as_bytes())?;
        writer.write_all(format!("IdToken: {}\n", &self.id_token).as_bytes())?;
        writer
            .write_all(format!("ServerKey: {}\n", base64::encode(&self.server_key)).as_bytes())?;
        Ok(())
    }
}

pub fn exists(alias: &str) -> bool {
    let mut path = std::path::PathBuf::new();
    path.push(dirs::home_dir().expect("Failed to get home directory"));
    path.push(HERMOD_BASE_DIR);
    path.push(HOST_DIR);
    path.push(alias);
    path.as_path().exists()
}

pub fn load_host(alias: &str) -> Result<Host, HermodError> {
    let mut path = PathBuf::new();
    path.push(dirs::home_dir().expect("Failed to get home directory"));
    path.push(HERMOD_BASE_DIR);
    path.push(HOST_DIR);
    path.push(alias);

    let mut host = Host::with_alias(alias);

    let file = std::fs::File::open(path)?;
    let buf_reader = std::io::BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        host = match parts[0] {
            "PublicKey:" => host.set_public_key(&base64::decode(parts[1])?),
            "PrivateKey:" => host.set_private_key(&base64::decode(parts[1])?),
            "Hostname:" => host.set_hostname(parts[1]),
            "IdToken:" => host.set_id_token(parts[1]),
            "ServerKey:" => host.set_server_key(&base64::decode(parts[1])?),
            _ => host,
        };
    }
    Ok(host)
}

pub async fn load_host_async(alias: &str) -> Result<Host, HermodError> {
    let mut path = PathBuf::new();
    path.push(dirs::home_dir().expect("Failed to get home directory"));
    path.push(HERMOD_BASE_DIR);
    path.push(HOST_DIR);
    path.push(alias);

    let mut host = Host::with_alias(alias);

    let file = File::open(path).await?;
    let mut lines = BufReader::new(file).lines();
    while let Some(line) = lines.next().await {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        host = match parts[0] {
            "PublicKey:" => host.set_public_key(&base64::decode(parts[1])?),
            "PrivateKey:" => host.set_private_key(&base64::decode(parts[1])?),
            "Hostname:" => host.set_hostname(parts[1]),
            "IdToken:" => host.set_id_token(parts[1]),
            "ServerKey:" => host.set_server_key(&base64::decode(parts[1])?),
            _ => host,
        };
    }
    Ok(host)
}
