use crate::config::{Config, SERVER_CONFIG};
use crate::consts::*;
use crate::error::{HermodError, HermodErrorKind};
use crate::genkey;
use crate::host::Host;
use crate::identity;
use crate::identity::KNOWN_CLIENTS;
use crate::message::{Message, MessageType};

use std::str;

use snow::{self, Builder, HandshakeState};

use async_std::net::TcpStream;
use async_std::prelude::*;

use futures::future::TryFutureExt;

pub async fn receive_key(stream: &mut TcpStream, msg: &Message) -> Result<(), HermodError> {
    println!("Reciving new key");
    let mut noise = Builder::new(SHARE_KEY_PATTERN.clone().parse()?)
        .local_private_key((*SERVER_CONFIG).get_private_key())
        .build_responder()?;

    let id = recv_identity(stream, &mut noise, msg).await?;

    // TODO: Add to KNOWN_CLIENT map
    identity::write_to_file(&id).await?;
    KNOWN_CLIENTS
        .lock()
        .await
        .insert(id.get_id().to_owned(), id);
    Ok(())
}

pub fn share_key(host: Host) {
    let keys = genkey::gen_keys().expect("Failed to generate a static key pair");
    let id = genkey::gen_idtoken();

    let mut noise = Builder::new(
        SHARE_KEY_PATTERN
            .clone()
            .parse()
            .expect("Invalid Noise Pattern supplied"),
    )
    .local_private_key(&keys.private)
    .build_initiator()
    .expect("Failed to create noise sate machine");

    async_std::task::block_on(async move {
        let socket_addr = (host.hostname(), HERMOD_PORT);

        println!("Connecting to {:?}", socket_addr);

        let server_key = match TcpStream::connect(socket_addr)
            .err_into::<HermodError>()
            .and_then(|stream| send_identity(stream, &mut noise, &id))
            .await
        {
            Ok(_) => noise
                .get_remote_static()
                .expect("Failed to read the remotes public static key")
                .to_vec(),
            Err(e) => {
                eprintln!("Failed to share key with the server: {}", e);
                return;
            }
        };

        let res = host
            .set_id_token(&id)
            .set_server_key(&server_key)
            .set_private_key(&keys.private)
            .set_public_key(&keys.public)
            .write_to_file()
            .map_err(|err| HermodError::new(HermodErrorKind::IoError(err)));

        match res {
            Ok(_) => println!("Succesfully shared keys with the remote"),
            Err(e) => eprintln!("Failed to write configuration to file: {}", e),
        }
    });
}

async fn recv_identity(
    stream: &mut TcpStream,
    noise: &mut HandshakeState,
    msg: &Message,
) -> Result<identity::Identity, HermodError> {
    // -> e
    noise.read_message(msg.get_payload(), &mut [])?;
    // <- e, s
    let mut buf = vec![0u8; HERMOD_KS_RES1_LEN];
    let len = noise.write_message(&[], &mut buf)?;
    stream.write_all(&[MessageType::ShareKeyResp as u8]).await?;
    stream.write_all(&buf[..len]).await?;
    // -> s, id
    let mut buf = vec![0u8; HERMOD_KS_RES2_LEN];
    let mut id = vec![0u8; ID_TOKEN_B64LEN as usize];
    stream.read_exact(&mut buf).await?;
    noise.read_message(&buf[MSG_TYPE_LEN..], &mut id)?;

    let id = identity::Identity::new(
        str::from_utf8(&id)
            .map_err(|_| HermodError::new(HermodErrorKind::Other))?
            .to_owned(),
        noise
            .get_remote_static()
            .expect("Failed to read the remotes public static key")
            .to_vec(),
    );
    stream.write_all(&[MessageType::Okey as u8]).await?;
    Ok(id)
}

async fn send_identity(
    mut stream: TcpStream,
    noise: &mut HandshakeState,
    token: &str,
) -> Result<(), HermodError> {
    // -> e
    let mut buf = vec![0u8; 64];
    let len = noise.write_message(&[], &mut buf)?;
    stream.write_all(&[MessageType::ShareKeyInit as u8]).await?;
    stream.write_all(&buf[..len]).await?;
    // <- e, s
    let mut buf = vec![0u8; HERMOD_KS_RES1_LEN];
    stream.read_exact(&mut buf).await?;
    noise.read_message(&buf[MSG_TYPE_LEN..], &mut [])?;
    // -> s, id
    let mut buf = vec![0u8; HERMOD_KS_RES2_LEN - MSG_TYPE_LEN];
    let len = noise.write_message(token.as_bytes(), &mut buf)?;
    stream.write_all(&[MessageType::ShareKeyResp as u8]).await?;
    stream.write_all(&buf[..len]).await?;
    // <- OK
    let mut buf = vec![0u8; MSG_TYPE_LEN];
    stream.read_exact(&mut buf).await?;
    match MessageType::from(buf[0]) {
        MessageType::Okey => Ok(()),
        _ => Err(HermodError::new(HermodErrorKind::ShareKey)),
    }
}
