use crate::config::SERVER_CONFIG;
use crate::error::HermodError;
use crate::message::{Message, MessageType};

static NOISE_PATTERN: &'static str = "NOISE_XX_25519_ChaChaPoly_BLAKE2s";

use snow::{self, Builder, HandshakeState, TransportState};

use async_std::net::TcpStream;

pub fn receive_key(stream: TcpStream, msg: &Message) -> Result<(), HermodError> {
    let mut noise = Builder::new(
        NOISE_PATTERN
            .clone()
            .parse()
            .expect("Failed to parse Noise Pattern"),
    )
    .local_private_key(SERVER_CONFIG.get_private_key())
    .build_initiator()?;
}
