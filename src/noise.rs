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
        let mut noise = Builder::new(NOISE_PATTERN.parse()?)
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
        let mut noise = Builder::new(NOISE_PATTERN.parse()?)
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
        let mut packet = [0u8; PACKET_MAXLENGTH];
        let msg_type = msg.get_type();
        let plaintext = msg.get_payload();
        let ciphertext_len = plaintext.len() + AEAD_TAG_LEN;

        // Generate new encryption key after sending 1GB of data
        if self.bytes_sent + ciphertext_len > REKEY_THRESHOLD {
            self.noise.rekey_outgoing();
            self.stream.write_all(&[MessageType::Rekey as u8]).await?;
            self.bytes_sent = 0;
            info!("Generating new session key");
        }

        let cipher_len = self
            .noise
            .write_message(plaintext, &mut packet[MSG_HEADER_LEN..])?;
        let len_arr = (cipher_len as u16).to_be_bytes();

        packet[0] = msg_type as u8;
        packet[1] = len_arr[0];
        packet[2] = len_arr[1];
        let msg_len = MSG_HEADER_LEN + cipher_len;

        self.stream.write_all(&packet[..msg_len]).await?;
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
        let mut length = [0u8; std::mem::size_of::<u16>()];
        self.stream.read_exact(&mut length).await?;
        let msg_len = u16::from_be_bytes(length) as usize;
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
    let mut packet = [0u8; PACKET_MAXLENGTH];

    packet[0] = MessageType::Init as u8;

    let mut i = 1;
    token.iter().for_each(|byte| {
        packet[i] = *byte;
        i += 1;
    });

    let _len = hs.write_message(&[], &mut packet[13..])?;
    stream
        .write_all(&packet[..HERMOD_HS_INIT_LEN])
        .await
        .unwrap();

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
    let mut resp_buffer = vec![0u8; 48 + MSG_TYPE_LEN];

    hs.read_message(&msg.get_payload()[12..], &mut [])?;
    let _len = hs.write_message(&[], &mut resp_buffer[MSG_TYPE_LEN..])?;
    resp_buffer[0] = MessageType::Response as u8;
    stream.write_all(&resp_buffer[..]).await?;
    Ok(())
}
