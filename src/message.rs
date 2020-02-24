use crate::consts::*;

use std::convert::From;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Init = 1,
    Response,
    Request,
    Payload,
    EOF,
    Error,
    Close,
    Okey,
    ShareKeyInit,
    ShareKeyResp,
    ShareIdentity,
    ShareHost,
    Unknown,
}

impl fmt::Debug for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Init => write!(f, "{}", "Init"),
            MessageType::Response => write!(f, "{}", "Response"),
            MessageType::Request => write!(f, "{}", "Request"),
            MessageType::Payload => write!(f, "{}", "Payload"),
            MessageType::EOF => write!(f, "{}", "EOF"),
            MessageType::Error => write!(f, "{}", "Error"),
            MessageType::Close => write!(f, "{}", "Close"),
            MessageType::Okey => write!(f, "Okey"),
            MessageType::ShareKeyInit => write!(f, "ShareKeyInit"),
            MessageType::ShareKeyResp => write!(f, "ShareKeyResp"),
            MessageType::ShareIdentity => write!(f, "ShareIdentity"),
            MessageType::ShareHost => write!(f, "ShareHost"),
            MessageType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Init => write!(f, "Init"),
            MessageType::Response => write!(f, "Response"),
            MessageType::Request => write!(f, "Request"),
            MessageType::Payload => write!(f, "Payload"),
            MessageType::EOF => write!(f, "EOF"),
            MessageType::Error => write!(f, "Error"),
            MessageType::Close => write!(f, "Close"),
            MessageType::Okey => write!(f, "Okey"),
            MessageType::ShareKeyInit => write!(f, "ShareKeyInit"),
            MessageType::ShareKeyResp => write!(f, "ShareKeyResp"),
            MessageType::ShareIdentity => write!(f, "ShareIdentity"),
            MessageType::ShareHost => write!(f, "ShareHost"),
            MessageType::Unknown => write!(f, "{}", "Unknown"),
        }
    }
}

impl From<u8> for MessageType {
    fn from(value: u8) -> MessageType {
        match value {
            1 => MessageType::Init,
            2 => MessageType::Response,
            3 => MessageType::Request,
            4 => MessageType::Payload,
            5 => MessageType::EOF,
            6 => MessageType::Error,
            7 => MessageType::Close,
            8 => MessageType::Okey,
            9 => MessageType::ShareKeyInit,
            10 => MessageType::ShareKeyResp,
            11 => MessageType::ShareIdentity,
            12 => MessageType::ShareHost,
            _ => MessageType::Unknown,
        }
    }
}

pub struct Message {
    msg_type: MessageType,
    payload: Vec<u8>,
}

impl Message {
    pub fn new(msg_type: MessageType, payload: &[u8]) -> Self {
        Message {
            msg_type,
            payload: payload.to_vec(),
        }
    }

    pub fn get_type(&self) -> MessageType {
        self.msg_type
    }

    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn len(&self) -> usize {
        MSG_TYPE_LEN + self.payload.len()
    }
}
