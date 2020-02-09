use crate::consts::*;

use snow::Builder;

pub fn gen_key() -> Result<snow::Keypair, snow::error::Error> {
    let builder = Builder::new(NOISE_PATTERN.parse());
    builder.generate_keypair()
}
