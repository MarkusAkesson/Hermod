use crate::config::Config;
use crate::consts::*;
use crate::message::{Message, MessageType};
use crate::peer::Peer;

use snow::{self, Builder, HandshakeState, TransportState};

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
}

impl NoiseStream {
    pub async fn new_initiator<C: Config>(
        peer: &Peer,
        config: &C,
        stream: &mut TcpStream,
    ) -> Result<Self, snow::error::Error> {
        let mut noise = NoiseStream::create(peer, config, NoiseRole::Initiator)?;

        client_handshake(stream, &mut noise).await?;

        let noise = noise.into_transport_mode()?;

        Ok(NoiseStream {
            stream: stream.to_owned(),
            noise,
        })
    }

    pub async fn new_responder<C: Config>(
        peer: &Peer,
        config: &C,
        stream: &mut TcpStream,
        message: &Message,
    ) -> Result<Self, snow::error::Error> {
        let mut noise = NoiseStream::create(peer, config, NoiseRole::Responder)?;

        server_handshake(stream, &mut noise, message).await?;

        let noise = noise.into_transport_mode()?;

        Ok(NoiseStream {
            stream: stream.to_owned(),
            noise,
        })
    }

    fn create<C: Config>(
        peer: &Peer,
        config: &C,
        role: NoiseRole,
    ) -> Result<HandshakeState, snow::error::Error> {
        let builder: Builder<'_> = Builder::new(NOISE_PATTERN.clone().parse()?)
            .local_private_key(config.get_private_key())
            .remote_public_key(peer.get_public_key())
            .prologue(peer.get_id().as_bytes());

        match role {
            NoiseRole::Initiator => builder.build_initiator(),
            NoiseRole::Responder => builder.build_responder(),
        }
    }

    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    pub async fn send(&mut self, msg: &Message) {
        let msg_type = msg.get_type();
        let plaintext = msg.get_payload();
        let mut ciphertext = vec![0u8; plaintext.len()];
        let msg_len = MSG_HEADER_LEN
            + self
                .noise
                .write_message(plaintext, &mut ciphertext)
                .unwrap();
        self.stream.write_all(&[msg_type as u8]).await.unwrap();
        self.stream.write_all(&msg_len.to_be_bytes()).await.unwrap();
        self.stream.write_all(&ciphertext).await.unwrap();
    }

    pub async fn recv(&mut self) -> Message {
        let mut msg_type = vec![0u8, MSG_HEADER_LEN as u8];
        self.stream.read_exact(&mut msg_type).await.unwrap();
        let mut length = [0u8, MSG_LENGTH_LEN as u8];
        self.stream.read_exact(&mut length).await.unwrap();
        let msg_len = u16::from_be_bytes(length) as usize;
        let mut enc_payload = vec![0u8; msg_len];
        self.stream.read_exact(&mut enc_payload).await.unwrap();
        let mut payload = vec![0u8; msg_len];
        self.noise.read_message(&enc_payload, &mut payload).unwrap();

        crate::message::Message::new(MessageType::from(msg_type[0]), &payload)
    }
}

async fn client_handshake(
    stream: &mut TcpStream,
    hs: &mut HandshakeState,
) -> Result<(), snow::error::Error> {
    let mut init_buffer = vec![0u8, HERMOD_HS_INIT_LEN as u8];
    let mut resp_buffer = vec![0u8, HERMOD_HS_RESP_LEN as u8];

    let msg_len = hs.write_message(&[], &mut init_buffer)?;
    stream.write_all(&init_buffer).await.unwrap();

    let mut read_buffer = vec![0u8, HERMOD_HS_RESP_LEN as u8];
    stream.read_exact(&mut read_buffer).await.unwrap();
    hs.read_message(&read_buffer, &mut resp_buffer)?;
    Ok(())
}

async fn server_handshake(
    stream: &mut TcpStream,
    hs: &mut HandshakeState,
    msg: &Message,
) -> Result<(), snow::error::Error> {
    let mut init_buffer = vec![0u8, HERMOD_HS_INIT_LEN as u8];
    let mut resp_buffer = vec![0u8, HERMOD_HS_RESP_LEN as u8];

    hs.read_message(&msg.get_payload(), &mut init_buffer)?;

    let msg_len = hs.write_message(&[], &mut resp_buffer)?;
    stream.write_all(&init_buffer).await.unwrap();
    Ok(())
}
