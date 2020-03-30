use crate::config::Config;
use crate::consts::*;
use crate::error::HermodError;
use crate::message::{Message, MessageType};
use crate::peer::Peer;

use snow::{self, Builder, HandshakeState, TransportState};

use log::info;

use async_std::net::TcpStream;
use async_std::prelude::*;

pub enum NoiseRole {
    Initiator,
    Responder,
}

pub enum NoiseMode {
    Handshake(snow::HandshakeState),
    Transport(snow::TransportState),
}

pub struct NoiseStream {
    stream: TcpStream,
    noise: TransportState,
    bytes_sent: usize,
}

impl<'cfg> NoiseStream {
    pub async fn new_initiator<C: Config<'cfg>>(
        peer: &Peer,
        config: &C,
        stream: &mut TcpStream,
    ) -> Result<Self, HermodError> {
        let mut noise = Builder::new(NOISE_PATTERN.clone().parse().unwrap())
            .local_private_key(config.get_private_key())
            .remote_public_key(peer.get_public_key())
            .build_initiator()?;
        client_handshake(stream, &mut noise, peer.get_id().as_bytes()).await?;

        let noise = noise.into_transport_mode()?;

        Ok(NoiseStream {
            stream: stream.to_owned(),
            noise,
            bytes_sent: 0,
        })
    }

    pub async fn new_responder<C: Config<'cfg>>(
        peer: &Peer,
        config: &C,
        stream: &mut TcpStream,
        message: &Message,
    ) -> Result<Self, HermodError> {
        let mut noise = Builder::new(NOISE_PATTERN.clone().parse()?)
            .local_private_key(config.get_private_key())
            .remote_public_key(peer.get_public_key())
            .build_responder()?;

        server_handshake(stream, &mut noise, message).await?;

        let noise = noise.into_transport_mode()?;

        Ok(NoiseStream {
            stream: stream.to_owned(),
            noise,
            bytes_sent: 0,
        })
    }

    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    pub async fn send(&mut self, msg: &Message) -> Result<(), HermodError> {
        let msg_type = msg.get_type();
        let plaintext = msg.get_payload();
        let mut ciphertext = vec![0u8; plaintext.len() + AEAD_TAG_LEN];

        // Generate new encryption key after sending 1GB of data
        if self.bytes_sent + ciphertext.len() > REKEY_THRESHOLD {
            self.noise.rekey_outgoing();
            self.stream.write_all(&[MessageType::Rekey as u8]).await?;
            self.bytes_sent = 0;
            info!("new key needed");
        }

        let cipher_len = self.noise.write_message(plaintext, &mut ciphertext)?;
        self.stream.write_all(&[msg_type as u8]).await?;
        self.stream
            .write_all(&(cipher_len as u32).to_be_bytes())
            .await?;
        self.stream.write_all(&ciphertext[..cipher_len]).await?;
        self.bytes_sent += cipher_len;
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Message, HermodError> {
        let mut msg_type = vec![0u8; MSG_TYPE_LEN];
        self.stream.read_exact(&mut msg_type).await?;
        if msg_type[0] == MessageType::Close as u8 {
            return Ok(Message::new(MessageType::Close, &[]));
        } else if msg_type[0] == MessageType::Rekey as u8 {
            self.stream.read_exact(&mut msg_type).await?;
            self.noise.rekey_incoming();
            info!("new key needed");
        }
        let mut length = [0u8; MSG_LENGTH_LEN];
        self.stream.read_exact(&mut length).await?;
        let msg_len = u32::from_be_bytes(length) as usize;
        let mut enc_payload = vec![0u8; msg_len];
        self.stream.read_exact(&mut enc_payload).await?;
        let mut payload = vec![0u8; msg_len - MAC_LENGTH];
        self.noise.read_message(&enc_payload, &mut payload)?;

        Ok(Message::new(MessageType::from(msg_type[0]), &payload))
    }
}

async fn client_handshake(
    stream: &mut TcpStream,
    hs: &mut HandshakeState,
    token: &[u8],
) -> Result<(), snow::error::Error> {
    let mut init_buffer = vec![0u8; 64];

    let len = hs.write_message(&[], &mut init_buffer)?;

    stream.write_all(&[MessageType::Init as u8]).await.unwrap();
    stream.write_all(token).await.unwrap();
    stream.write_all(&init_buffer[..len]).await.unwrap();

    let mut read_buffer = vec![0u8; HERMOD_HS_RESP_LEN + MSG_TYPE_LEN];
    let mut resp_buffer = vec![0u8; HERMOD_HS_RESP_LEN];
    stream.read_exact(&mut read_buffer).await.unwrap();
    hs.read_message(&read_buffer[MSG_TYPE_LEN..], &mut resp_buffer)?;
    Ok(())
}

async fn server_handshake(
    stream: &mut TcpStream,
    hs: &mut HandshakeState,
    msg: &Message,
) -> Result<(), HermodError> {
    let mut resp_buffer = vec![0u8; 64];

    hs.read_message(&msg.get_payload()[12..], &mut [])?;

    let len = hs.write_message(&[], &mut resp_buffer)?;
    stream.write_all(&[MessageType::Response as u8]).await?;
    stream.write_all(&resp_buffer[..len]).await?;
    Ok(())
}
