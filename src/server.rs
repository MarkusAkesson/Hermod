use crate::consts::*;
use crate::error::HermodError;
use crate::genkey;
use crate::identity;
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;
use crate::share_key;

use std::fs::{self, File};
use std::io::prelude::*;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str;
use std::time::Duration;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

use async_listen::{backpressure::Token, error_hint, ListenExt};

use daemonize::Daemonize;

use log::{debug, error, info, warn};

pub struct Server {
    workers: u8,
    socket_addr: SocketAddr,
}

impl<'hs> Server {
    pub fn new(ip: SocketAddr, workers: u8) -> Self {
        Self {
            workers,
            socket_addr: ip,
        }
    }

    pub fn daemonize(&self) -> Result<(), HermodError> {
        let stdout = File::create("/tmp/hermod.out")?;
        let stderr = File::create("/tmp/hermod.err")?;
        let daemon = Daemonize::new()
            .pid_file("/tmp/hermod.pid")
            .working_directory(dirs::home_dir().expect("Could not find the home directory"))
            .stdout(stdout)
            .stderr(stderr);

        daemon.start().map_err(|e| HermodError::DaemonError(e))
    }

    pub fn start(&self) {
        async_std::task::block_on(async {
            let listener: TcpListener = TcpListener::bind(self.socket_addr).await.unwrap();
            info!("Listening on {}", listener.local_addr().unwrap());

            let mut incoming = listener
                .incoming()
                .log_warnings(|e| {
                    warn!("Accept error: {}. Sleeping 0.5s. {}", e, error_hint(&e));
                })
                .handle_errors(Duration::from_millis(500))
                .backpressure(100);

            while let Some((token, mut stream)) = incoming.next().await {
                task::spawn(async move {
                    match handle_connection(&token, &mut stream).await {
                        Ok(_) => return,
                        Err(e) => {
                            error!("{}", e);
                            return;
                        }
                    }
                });
            }
        });
    }

    pub fn setup(force: bool) {
        let exists = |filepath: &str| -> bool {
            let mut path = PathBuf::new();
            path.push(dirs::home_dir().expect("Failed to get home directory"));
            path.push(HERMOD_BASE_DIR);
            path.push(filepath);
            path.as_path().exists()
        };

        let exists = exists(SERVER_PRIVATE_KEY_FILE) || exists(SERVER_PUBLIC_KEY_FILE);

        if exists && !force {
            error!("Previous configuration found, pass --force to overwrite");
            return;
        } else if exists && force {
            info!("Existing configuration found, overwriting");
        }

        let keys =
            genkey::create_server_keys().expect("Failed to crate static keys for the server");

        let write_key = |key: &[u8], filepath: &str| -> io::Result<()> {
            let mut path = PathBuf::new();
            path.push(dirs::home_dir().expect("Failed to get home directory"));
            path.push(HERMOD_BASE_DIR);
            fs::create_dir_all(&path).expect("Failed to create hermod directory directory");
            path.push(filepath);
            let mut file = File::create(path)?;
            file.write_all(base64::encode(key).as_bytes())?;
            Ok(())
        };

        write_key(&keys.private, SERVER_PRIVATE_KEY_FILE)
            .expect("Failed to write the private key to file");
        write_key(&keys.public, SERVER_PUBLIC_KEY_FILE)
            .expect("Failed to write the public key to file");
    }

    pub fn list_known_clients() {
        identity::print_known_clients();
    }
}

async fn handle_connection(_token: &Token, stream: &mut TcpStream) -> Result<(), HermodError> {
    // log incomming packet from ip

    let mut msg_type = vec![0u8];
    stream.read_exact(&mut msg_type).await?;
    debug!(
        "Incomming message of type: {}, value: {:?}",
        MessageType::from(msg_type[0]),
        msg_type
    );
    match MessageType::from(msg_type[0]) {
        MessageType::Init => incomming_request(stream).await,
        MessageType::ShareKeyInit => share_key(stream).await,
        _ => Ok(()),
    }
}

async fn share_key(stream: &mut TcpStream) -> Result<(), HermodError> {
    debug!("Sharing key with client");
    let mut buffer = vec![0u8; HERMOD_KS_INIT_LEN];
    stream.read_exact(&mut buffer).await?;
    let msg = Message::new(MessageType::ShareKeyInit, &buffer);
    share_key::receive_key(stream, &msg).await?;
    debug!("Shared key with client");
    Ok(())
}

async fn incomming_request(stream: &mut TcpStream) -> Result<(), HermodError> {
    let mut buffer = vec![0u8; HERMOD_HS_INIT_LEN - MSG_TYPE_LEN];
    stream.read_exact(&mut buffer).await?;

    let msg = Message::new(MessageType::Init, &buffer);
    // 12 = tokenid base64len
    let peer = Peer::new_client_peer(
        &str::from_utf8(&msg.get_payload()[0..12])
            .expect("Failed to read client id from Init message"),
    )
    .await?;

    let mut endpoint = Endpoint::server(stream, peer, &msg).await?;

    // Request loop listen for and handle incomming requests
    loop {
        let msg = match endpoint.recv().await {
            Ok(msg) => msg,
            Err(e) => {
                error!("Connection error: {}", e);
                break;
            }
        };

        match msg.get_type() {
            MessageType::Error => {
                error!("Received 'error' from the client, closing connection");
                break;
            }
            MessageType::Request => {
                let request: Request = bincode::deserialize(msg.get_payload()).unwrap();
                request.respond(&mut endpoint).await?
            }
            MessageType::Close => {
                info!("Received 'close' from the client, closing connection");
                break;
            }
            _ => {
                error!("Received unexpected message from the client, closing connection");
                break;
            } // log: Received message out of order {} type, Closing connection
        }
    }
    info!("Closing connection");
    Ok(())
}
