use crate::consts::*;
use crate::error::HermodError;
use crate::genkey;
use crate::identity;
use crate::message::{Message, MessageType};
use crate::peer::Endpoint;
use crate::peer::Peer;
use crate::request::Request;

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
                    handle_connection(&mut stream).await.unwrap();
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

    // TODO: Clean up
    // 13 = tokenid base64len +  MessageType
    let mut buffer = vec![0u8; HERMOD_HS_INIT_LEN + 13];
    stream.read_exact(&mut buffer).await.unwrap();
    let msg = Message::new(MessageType::from(buffer[0]), &buffer[1..]);

    let peer = match msg.get_type() {
        MessageType::Init => {
            Peer::new_client_peer(&str::from_utf8(&msg.get_payload()[0..12]).unwrap())
        }
        _ => return Ok(()),
    };

    let mut endpoint = Endpoint::server(stream, peer, &msg).await?;

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
            MessageType::Request => process_incomming_request(&msg, &mut endpoint).await,
            MessageType::Close => break,
            MessageType::Payload
            | MessageType::Unknown
            | MessageType::Init
            | MessageType::Response
            | MessageType::EOF => break, // log: Received message out of order {} type, Closing connection
        }
    }
    println!("Closing connection");
    Ok(())
}

async fn process_incomming_request(msg: &Message, endpoint: &mut Endpoint) {
    let request: Request = bincode::deserialize(msg.get_payload()).unwrap();
    request.respond(endpoint).await;
}
