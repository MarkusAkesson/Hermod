#[derive(Eq, PartialEq)]
pub enum MessageType {
    Init,
    Response,
    Request,
    Payload,
    Error,
}

pub fn parse_message_type(value: u8) -> MessageType {
    match value {
        0x1 => MessageType::Init,
        0x2 => MessageType::Response,
        0x3 => MessageType::Request,
        0x4 => MessageType::Payload,
        0x5 => MessageType::Error,
    }
}

pub struct Init {
    pub msg_type: MessageType,
    pub id: String,
    pub noise_payload: Vec<u8>,
}

impl Init {
    pub fn new(buffer: &[u8]) -> Self {
        let msg_type = parse_message_type(buffer[0]);
        let id = String::from_utf8_lossy(buffer[1..5]);
        let noise_payload = buffer[6..].to_owned();

        Init {
            msg_type,
            id,
            noise_payload,
        }
    }
}
