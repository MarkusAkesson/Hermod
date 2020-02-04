pub struct Peer {
    id: String,
    public_key: Vec<u8>,
}

impl Peer {
    pub fn new(id: &str, public_key: &[u8]) -> Self {
        Peer {
            id: id.to_string(),
            public_key: public_key.to_vec(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }
}
