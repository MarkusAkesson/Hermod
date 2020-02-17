use crate::consts::*;

use snow::Builder;

pub fn create_server_keys() -> Result<snow::Keypair, snow::error::Error> {
    let builder = Builder::new(NOISE_PATTERN.parse()?);
    builder.generate_keypair()
}

pub fn gen_keys() -> Result<snow::Keypair, snow::error::Error> {
    let builder = Builder::new(NOISE_PATTERN.parse()?);
    builder.generate_keypair()
}
