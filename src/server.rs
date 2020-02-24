use crate::consts::*;
use crate::error::HermodError;
use crate::genkey;
use crate::identity;
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;
use crate::share_key;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

pub struct HermodServer {}

impl<'hs> HermodServer {
    pub fn run_server() {
        async_std::task::block_on(async {
            let listener: TcpListener = TcpListener::bind("127.0.0.1:4444").await.unwrap();
            println!("Listening on {}", listener.local_addr().unwrap());

            let mut incoming = listener.incoming();
            while let Some(stream) = incoming.next().await {
                task::spawn(async {
                    let mut stream = stream.unwrap();
                    match handle_connection(&mut stream).await {
                        Ok(_) => return,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }
                });
            }
        });
    }

    pub fn setup() {
        let keys = genkey::create_server_keys().unwrap();

        let write_to_file = |key: &[u8], filepath: &str| -> io::Result<()> {
            let mut path = PathBuf::new();
            path.push(dirs::home_dir().unwrap());
            path.push(filepath);
            let mut file = File::create(path).unwrap();
            file.write_all(base64::encode(key).as_bytes())?;
            Ok(())
        };

        write_to_file(&keys.private, SERVER_PRIVATE_KEY_FILE).unwrap();
        write_to_file(&keys.public, SERVER_PUBLIC_KEY_FILE).unwrap();
    }

    pub fn list_known_clients() {
        identity::print_known_clients();
    }
}

async fn handle_connection(stream: &mut TcpStream) -> Result<(), HermodError> {
    // log incomming packet from ip

    let mut msg_type = vec![0u8];
    stream.read_exact(&mut msg_type).await?;
    println!(
        "Incomming message of type: {}, value: {:?}",
        MessageType::from(msg_type[0]),
        msg_type
    );
    match MessageType::from(msg_type[0]) {
        MessageType::Init => incomming_request(stream).await,
        MessageType::ShareKeyInit => share_key(stream).await,
        _ => return Ok(()),
    }
}

async fn share_key(stream: &mut TcpStream) -> Result<(), HermodError> {
    println!("Sharing key with client");
    let mut buffer = vec![0u8; HERMOD_KS_INIT_LEN];
    stream.read_exact(&mut buffer).await?;
    let msg = Message::new(MessageType::ShareKeyInit, &buffer);
    share_key::receive_key(stream, &msg).await?;
    println!("Shared key with client");
    Ok(())
}

async fn incomming_request(stream: &mut TcpStream) -> Result<(), HermodError> {
    // TODO: Clean up
    // 12 = tokenid base64len
    let mut buffer = vec![0u8; HERMOD_HS_INIT_LEN + 12];
    stream.read_exact(&mut buffer).await?;

    let msg = Message::new(MessageType::Init, &buffer);

    let peer = Peer::new_client_peer(
        &str::from_utf8(&msg.get_payload()[0..12])
            .expect("Failed to read client id from Init message"),
    );

    let mut endpoint = Endpoint::server(stream, peer, &msg).await?;

    // Request loop listen for and handle incomming request
    loop {
        let msg = match endpoint.recv().await {
            Ok(msg) => msg,
            Err(e) => {
                println!("{}", e);
                break;
            }
        };

        match msg.get_type() {
            MessageType::Error => break, // Received error, log error message, Cloe Connection
            MessageType::Request => {
                let request: Request = bincode::deserialize(msg.get_payload()).unwrap();
                request.respond(&mut endpoint).await?
            }
            MessageType::Close => break,
            _ => break, // log: Received message out of order {} type, Closing connection
        }
    }
    println!("Closing connection");
    Ok(())
}
