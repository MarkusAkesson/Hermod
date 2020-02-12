use crate::consts::*;

use std::convert::From;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Init = 1,
    Response,
    Request,
    Payload,
    EOF,
    Error,
    Unknown,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> MessageType {
        match value {
            0x1 => MessageType::Init,
            0x2 => MessageType::Response,
            0x3 => MessageType::Request,
            0x4 => MessageType::Payload,
            0x5 => MessageType::EOF,
            0x6 => MessageType::Error,
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
