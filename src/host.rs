pub struct Host {
    pub hostname: String,
    pub id_token: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub server_key: Vec<u8>,
}

impl Host {}
