use std::error;
use std::fmt;

#[derive(Debug)]
pub struct HermodError(Box<HermodErrorKind>);
#[derive(Debug)]
pub enum HermodErrorKind {
    FileNotFound(std::io::Error),
    OpenFile(std::io::Error),
    CreateFile(std::io::Error),
    ConnectionRefused(std::io::Error),
    UnknownMessage,
    UnknownIdentity,
    UnknownHost,
    IoError(std::io::Error),
    B64Decode(base64::DecodeError),
    ReadPubKey(std::io::Error),
    ReadPrivKey(std::io::Error),
    ReadAuthorizedClients(std::io::Error),
    OutOfOrderMessage,
    Snow(snow::error::Error),
    Other,
}

impl HermodError {
    pub fn new(kind: HermodErrorKind) -> HermodError {
        HermodError(Box::new(kind))
    }

    pub fn kind(&self) -> &HermodErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> HermodErrorKind {
        *self.0
    }
}

impl fmt::Display for HermodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.0 {
            HermodErrorKind::FileNotFound(ref err) => write!(f, "Could not find file: {}", err),
            HermodErrorKind::OpenFile(ref err) => write!(f, "Could not open file: {}", err),
            HermodErrorKind::CreateFile(ref err) => write!(f, "Could not create file: {}", err),
            HermodErrorKind::ConnectionRefused(ref err) => {
                write!(f, "Could not connect to host: {}", err)
            }
            HermodErrorKind::UnknownMessage => write!(f, "Received unknown message"),
            HermodErrorKind::UnknownIdentity => {
                write!(f, "Authentication attempt form unknown identity")
            }
            HermodErrorKind::UnknownHost => write!(f, "The host  was not found in knwon_hosts"),
            HermodErrorKind::IoError(ref err) => write!(f, "{}", err),
            HermodErrorKind::B64Decode(ref err) => {
                write!(f, "Failed to decode from base64: {}", err)
            }
            HermodErrorKind::ReadPubKey(ref err) => {
                write!(f, "Failed to read the public server key: {}", err)
            }
            HermodErrorKind::ReadPrivKey(ref err) => {
                write!(f, "Failed to read the private server key: {}", err)
            }
            HermodErrorKind::ReadAuthorizedClients(ref err) => {
                write!(f, "Failed to read the authorized_clients file: {}", err)
            }
            HermodErrorKind::OutOfOrderMessage => write!(f, "Received unexpected message"),
            HermodErrorKind::Snow(ref err) => write!(f, "{}", err),
            HermodErrorKind::Other => write!(f, "Unspecified error"),
        }
    }
}

impl error::Error for HermodError {
    fn description(&self) -> &str {
        match *self.0 {
            HermodErrorKind::FileNotFound(ref err) => error::Error::description(err),
            HermodErrorKind::OpenFile(ref err) => error::Error::description(err),
            HermodErrorKind::CreateFile(ref err) => error::Error::description(err),
            HermodErrorKind::ConnectionRefused(ref err) => error::Error::description(err),
            HermodErrorKind::IoError(ref err) => error::Error::description(err),
            HermodErrorKind::B64Decode(ref err) => error::Error::description(err),
            HermodErrorKind::ReadPubKey(ref err) => error::Error::description(err),
            HermodErrorKind::ReadPrivKey(ref err) => error::Error::description(err),
            HermodErrorKind::ReadAuthorizedClients(ref err) => error::Error::description(err),
            _ => "TODO: Description for all enum values for hermod error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self.0 {
            HermodErrorKind::OpenFile(ref err) => Some(err),
            HermodErrorKind::CreateFile(ref err) => Some(err),
            HermodErrorKind::ConnectionRefused(ref err) => Some(err),
            HermodErrorKind::IoError(ref err) => Some(err),
            HermodErrorKind::B64Decode(ref err) => Some(err),
            HermodErrorKind::ReadPubKey(ref err) => Some(err),
            HermodErrorKind::ReadPrivKey(ref err) => Some(err),
            HermodErrorKind::ReadAuthorizedClients(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<async_std::io::Error> for HermodError {
    fn from(err: async_std::io::Error) -> HermodError {
        match err.kind() {
            async_std::io::ErrorKind::ConnectionRefused => {
                HermodError::new(HermodErrorKind::ConnectionRefused(err))
            }
            async_std::io::ErrorKind::NotFound => {
                HermodError::new(HermodErrorKind::FileNotFound(err))
            }
            async_std::io::ErrorKind::PermissionDenied
            | async_std::io::ErrorKind::ConnectionReset
            | async_std::io::ErrorKind::ConnectionAborted
            | async_std::io::ErrorKind::NotConnected
            | async_std::io::ErrorKind::AddrInUse
            | async_std::io::ErrorKind::AddrNotAvailable
            | async_std::io::ErrorKind::BrokenPipe
            | async_std::io::ErrorKind::AlreadyExists
            | async_std::io::ErrorKind::WouldBlock
            | async_std::io::ErrorKind::InvalidInput
            | async_std::io::ErrorKind::InvalidData
            | async_std::io::ErrorKind::TimedOut
            | async_std::io::ErrorKind::WriteZero
            | async_std::io::ErrorKind::Interrupted
            | async_std::io::ErrorKind::Other
            | async_std::io::ErrorKind::UnexpectedEof => {
                HermodError::new(HermodErrorKind::IoError(err))
            }

            _ => HermodError::new(HermodErrorKind::Other),
        }
    }
}

impl From<base64::DecodeError> for HermodError {
    fn from(err: base64::DecodeError) -> HermodError {
        HermodError::new(HermodErrorKind::B64Decode(err))
    }
}

impl From<snow::error::Error> for HermodError {
    fn from(err: snow::error::Error) -> HermodError {
        HermodError::new(HermodErrorKind::Snow(err))
    }
}
