use crate::consts::*;

use std::convert::From;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Init = 1,
    Response,
    Request,
    Payload,
    Metadata,
    EOF,
    Error,
    Close,
    Okay,
    ShareKeyInit,
    ShareKeyResp,
    ShareIdentity,
    ShareHost,
    EndOfResponse,
    Rekey,
    Unknown,
}

impl fmt::Debug for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Init => write!(f, "Init"),
            MessageType::Response => write!(f, "Response"),
            MessageType::Request => write!(f, "Request"),
            MessageType::Payload => write!(f, "Payload"),
            MessageType::Metadata => write!(f, "Metadata"),
            MessageType::EOF => write!(f, "EOF"),
            MessageType::Error => write!(f, "Error"),
            MessageType::Close => write!(f, "Close"),
            MessageType::Okay => write!(f, "Okay"),
            MessageType::ShareKeyInit => write!(f, "ShareKeyInit"),
            MessageType::ShareKeyResp => write!(f, "ShareKeyResp"),
            MessageType::ShareIdentity => write!(f, "ShareIdentity"),
            MessageType::ShareHost => write!(f, "ShareHost"),
            MessageType::EndOfResponse => write!(f, "EndOfResponse"),
            MessageType::Rekey => write!(f, "Rekey"),
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
            MessageType::Metadata => write!(f, "Metadata"),
            MessageType::EOF => write!(f, "EOF"),
            MessageType::Error => write!(f, "Error"),
            MessageType::Close => write!(f, "Close"),
            MessageType::Okay => write!(f, "Okay"),
            MessageType::ShareKeyInit => write!(f, "ShareKeyInit"),
            MessageType::ShareKeyResp => write!(f, "ShareKeyResp"),
            MessageType::ShareIdentity => write!(f, "ShareIdentity"),
            MessageType::ShareHost => write!(f, "ShareHost"),
            MessageType::EndOfResponse => write!(f, "EndOfResponse"),
            MessageType::Rekey => write!(f, "Rekey"),
            MessageType::Unknown => write!(f, "Unknown"),
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
            5 => MessageType::Metadata,
            6 => MessageType::EOF,
            7 => MessageType::Error,
            8 => MessageType::Close,
            9 => MessageType::Okay,
            10 => MessageType::ShareKeyInit,
            11 => MessageType::ShareKeyResp,
            12 => MessageType::ShareIdentity,
            13 => MessageType::ShareHost,
            14 => MessageType::EndOfResponse,
            15 => MessageType::EndOfResponse,
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

    pub fn is_empty(&self) -> bool {
        self.payload.len() == 0
    }
}
