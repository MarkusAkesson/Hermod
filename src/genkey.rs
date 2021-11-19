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

#[cfg(test)]
mod tests {
    use crate::genkey::*;

    #[test]
    fn diff_gen_keys() {
        let key1 = gen_keys().unwrap();
        let key2 = gen_keys().unwrap();

        assert_ne!(key1.private, key2.private);
        assert_ne!(key1.public, key2.public);
    }

    #[test]
    fn diff_server_keys() {
        let key1 = create_server_keys().unwrap();
        let key2 = create_server_keys().unwrap();

        assert_ne!(key1.private, key2.private);
        assert_ne!(key1.public, key2.public);
    }

    #[test]
    fn diff_gen_idtoken() {
        let token1 = gen_idtoken();
        let token2 = gen_idtoken();

        assert_ne!(token1, token2);
    }
}
