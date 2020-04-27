use crate::consts::*;

use rand::prelude::*;
use snow::Builder;

pub fn create_server_keys() -> Result<snow::Keypair, snow::error::Error> {
    let builder = Builder::new(NOISE_PATTERN.parse()?);
    builder.generate_keypair()
}

pub fn gen_keys() -> Result<snow::Keypair, snow::error::Error> {
    let builder = Builder::new(NOISE_PATTERN.parse()?);
    builder.generate_keypair()
}

pub fn gen_idtoken() -> String {
    let mut vec = [0u8; ID_TOKEN_LEN as usize];
    rand::thread_rng().fill_bytes(&mut vec);
    base64::encode(&vec)
}
