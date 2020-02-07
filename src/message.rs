#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MessageType {
    Init,
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

impl Message {
    pub fn new(buffer: &[u8]) -> Result<Self, &'static str> {
        let msg_type = parse_message_type(buffer[0]);
        let msg = match msg_type {
            MessageType::Init => Message::Init(InitMessage::new(buffer)),
            MessageType::Response => Message::Response(ResponseMessage::new(buffer)),
            MessageType::Request => Message::Request(RequestMessage::new(buffer)),
            MessageType::Payload => Message::Payload(PayloadMessage::new(buffer)),
            MessageType::Error => Message::Error(ErrorMessage::new(buffer)),
            MessageType::Unknown => unimplemented!(),
        };
        Ok(msg)
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

    pub fn as_bytes(&self) -> &[u8] {
        unimplemented!()
    }
}

pub fn parse_message_type(value: u8) -> MessageType {
    match value {
        0x1 => MessageType::Init,
        0x2 => MessageType::Response,
        0x3 => MessageType::Request,
        0x4 => MessageType::Payload,
        0x5 => MessageType::Error,
        _ => MessageType::Unknown,
    }
}

pub struct InitMessage {
    pub msg_type: MessageType,
    pub id: String,
    pub noise_payload: Vec<u8>,
}

impl InitMessage {
    pub fn new(buffer: &[u8]) -> Self {
        let msg_type = MessageType::Init;
        let id = String::from_utf8_lossy(&buffer[1..5]);
        let noise_payload = buffer[6..].to_owned();

        InitMessage {
            msg_type,
            id: id.into_owned(),
            noise_payload,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

pub struct ResponseMessage {
    pub msg_type: MessageType,
}

impl ResponseMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}

pub struct RequestMessage {
    pub msg_type: MessageType,
}

impl RequestMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}

pub struct PayloadMessage {
    pub msg_type: MessageType,
}

impl PayloadMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}

pub struct ErrorMessage {
    pub msg_type: MessageType,
}

impl ErrorMessage {
    pub fn new(buffer: &[u8]) -> Self {
        unimplemented!();
    }
}
