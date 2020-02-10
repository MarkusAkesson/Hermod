use std::convert::From;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Init = 1,
    Response,
    Request,
    Payload,
    Error,
    Unknown,
}

pub enum Message {
    Init(InitMessage),
    Response(ResponseMessage),
    Request(RequestMessage),
    Payload(PayloadMessage),
    Error(ErrorMessage),
}

impl From<u8> for MessageType {
    fn from(value: u8) -> MessageType {
        match value {
            0x1 => MessageType::Init,
            0x2 => MessageType::Response,
            0x3 => MessageType::Request,
            0x4 => MessageType::Payload,
            0x5 => MessageType::Error,
            _ => MessageType::Unknown,
        }
    }
}

impl Message {
    pub fn from_buffer(msg_type: u8, buffer: &[u8]) -> Result<Self, &'static str> {
        let msg = match MessageType::from(msg_type) {
            MessageType::Init => Message::Init(InitMessage::from_buffer(buffer)),
            MessageType::Response => Message::Response(ResponseMessage::from_buffer(buffer)),
            MessageType::Request => Message::Request(RequestMessage::from_buffer(buffer)),
            MessageType::Payload => Message::Payload(PayloadMessage::from_buffer(buffer)),
            MessageType::Error => Message::Error(ErrorMessage::from_buffer(buffer)),
            MessageType::Unknown => unimplemented!(),
        };
        Ok(msg)
    }

    pub fn new(buffer: &[u8], msg_type: MessageType) -> Self {
        let msg = match msg_type {
            MessageType::Init => Message::Init(InitMessage::new(buffer)),
            MessageType::Response => Message::Response(ResponseMessage::new(buffer)),
            MessageType::Request => Message::Request(RequestMessage::new(buffer)),
            MessageType::Payload => Message::Payload(PayloadMessage::new(buffer)),
            MessageType::Error => Message::Error(ErrorMessage::new(buffer)),
            MessageType::Unknown => unreachable!(),
        };
        msg
    }

    pub fn get_type(&self) -> MessageType {
        match self {
            Message::Init(msg) => msg.msg_type,
            Message::Response(msg) => msg.msg_type,
            Message::Request(msg) => msg.msg_type,
            Message::Payload(msg) => msg.msg_type,
            Message::Error(msg) => msg.msg_type,
        }
    }

    pub fn len(&self) -> usize {
        unimplemented!();
    }

    pub fn get_payload(&self) -> &[u8] {
        match self {
            Message::Init(msg) => &msg.payload,
            Message::Response(msg) => &msg.payload,
            Message::Request(msg) => &msg.payload,
            Message::Payload(msg) => &msg.payload,
            Message::Error(msg) => &msg.payload,
        }
    }

    pub fn process(&self) -> Option<Message> {
        match self {
            Message::Init(msg) => None,
            Message::Response(msg) => None,
            Message::Request(msg) => None,
            Message::Payload(msg) => None,
            Message::Error(msg) => None,
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        unimplemented!()
    }
}

pub struct InitMessage {
    pub msg_type: MessageType,
    pub id: String,
    pub payload: Vec<u8>,
}

impl InitMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        let msg_type = MessageType::Init;
        let id = String::from_utf8_lossy(&buffer[1..5]);
        let payload = buffer[6..].to_owned();

        InitMessage {
            msg_type,
            id: id.into_owned(),
            payload,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

pub struct ResponseMessage {
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

impl ResponseMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}

pub struct RequestMessage {
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

impl RequestMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}

pub struct PayloadMessage {
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

impl PayloadMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}

pub struct ErrorMessage {
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

impl ErrorMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}
