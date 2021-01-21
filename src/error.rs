use thiserror::Error;

#[derive(Error, Debug)]
pub enum HermodError {
    #[error("Expected a file, found a directory")]
    IsDir,
    #[error("Unknown message type")]
    UnknownMessage,
    #[error("Authentication attempt from unknown identity")]
    UnknownIdentity,
    #[error("Host was not found in known_hosts")]
    UnknownHost,
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    B64Decode(#[from] base64::DecodeError),
    #[error("Unexpected message")]
    UnexpectedMessage,
    #[error("{0}")]
    Snow(#[from] snow::error::Error),
    #[error("{0}")]
    Channel(#[from] async_std::channel::SendError<crate::message::Message>),
    #[error("{0}")]
    DaemonError(#[from] daemonize::DaemonizeError),
    #[error("Unknown error")]
    Other,
}
