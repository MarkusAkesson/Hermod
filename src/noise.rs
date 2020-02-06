use crate::config::Config;
use crate::peer::Peer;

use snow::{self, Builder};

static NOISE_PATTERN: &'static str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";

pub enum NoiseMode {
    Handshake(snow::HandshakeState),
    Transport(snow::TransportState),
}

pub enum NoiseRole {
    Initiator,
    Responder,
}

pub struct NoiseSession {
    noise: NoiseMode,
}

impl NoiseMode {
    pub fn read_message(
        &mut self,
        message: &[u8],
        payload: &mut [u8],
    ) -> Result<usize, snow::error::Error> {
        let len = match self {
            NoiseMode::Handshake(session) => session.read_message(message, payload)?,
            NoiseMode::Transport(session) => session.read_message(message, payload)?,
        };

        Ok(len)
    }

    pub fn write_message(
        &mut self,
        message: &[u8],
        payload: &mut [u8],
    ) -> Result<usize, snow::error::Error> {
        let len = match self {
            NoiseMode::Handshake(session) => session.write_message(message, payload)?,
            NoiseMode::Transport(session) => session.write_message(message, payload)?,
        };

        Ok(len)
    }

    pub fn into_transport_mode(self) -> Result<snow::TransportState, snow::error::Error> {
        match self {
            NoiseMode::Handshake(session) => session.into_transport_mode(),
            NoiseMode::Transport(_) => {
                return Err(snow::Error::State(
                    snow::error::StateProblem::HandshakeAlreadyFinished,
                ))
            }
        }
    }
}

impl NoiseSession {
    pub fn new(
        peer: &Peer,
        config: &impl Config,
        role: NoiseRole,
    ) -> Result<Self, snow::error::Error> {
        let builder: Builder<'_> = Builder::new(NOISE_PATTERN.clone().parse()?)
            .local_private_key(config.get_private_key())
            .remote_public_key(peer.get_public_key())
            .prologue(peer.get_id().as_bytes());

        let state = match role {
            NoiseRole::Initiator => NoiseMode::Handshake(builder.build_initiator()?),
            NoiseRole::Responder => NoiseMode::Handshake(builder.build_responder()?),
        };

        Ok(NoiseSession { noise: state })
    }
    pub fn read_message(
        &mut self,
        message: &[u8],
        payload: &mut [u8],
    ) -> Result<usize, snow::error::Error> {
        self.noise.read_message(message, payload)
    }

    pub fn write_message(
        &mut self,
        message: &[u8],
        payload: &mut [u8],
    ) -> Result<usize, snow::error::Error> {
        self.noise.write_message(message, payload)
    }
}
