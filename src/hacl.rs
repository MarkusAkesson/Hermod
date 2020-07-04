use std::convert::TryInto;

use skymd::{chacha20poly1305, curve25519};
use snow::{
    params::{CipherChoice, DHChoice, HashChoice},
    resolvers::CryptoResolver,
    types::{Cipher, Dh, Hash, Random},
};

#[derive(Default)]
pub struct HaclResolver;

impl CryptoResolver for HaclResolver {
    fn resolve_rng(&self) -> Option<Box<dyn Random>> {
        None
    }

    fn resolve_dh(&self, choice: &DHChoice) -> Option<Box<dyn Dh>> {
        match *choice {
            DHChoice::Curve25519 => Some(Box::new(HaclCurve25519::default())),
            _ => None,
        }
    }

    fn resolve_cipher(&self, choice: &CipherChoice) -> Option<Box<dyn Cipher>> {
        match *choice {
            CipherChoice::ChaChaPoly => Some(Box::new(HaclChachaPoly::default())),
            _ => None,
        }
    }

    fn resolve_hash(&self, _choice: &HashChoice) -> Option<Box<dyn Hash>> {
        None
    }
}

pub struct HaclCurve25519 {
    privkey: curve25519::PrivateKey,
    pubkey: curve25519::PublicKey,
}

impl Default for HaclCurve25519 {
    fn default() -> Self {
        skymd::skymd_init();

        Self {
            privkey: curve25519::PrivateKey::default(),
            pubkey: curve25519::PublicKey::default(),
        }
    }
}

impl Dh for HaclCurve25519 {
    fn name(&self) -> &'static str {
        "25519"
    }

    fn pub_len(&self) -> usize {
        curve25519::KEY_LEN
    }

    fn priv_len(&self) -> usize {
        curve25519::KEY_LEN
    }

    fn set(&mut self, privkey: &[u8]) {
        self.privkey = curve25519::PrivateKey::from_slice(
            privkey.try_into().expect("Wrong length on the private key"),
        );
        self.pubkey = curve25519::PublicKey::from(&self.privkey);
    }

    fn pubkey(&self) -> &[u8] {
        self.pubkey.as_bytes()
    }

    fn privkey(&self) -> &[u8] {
        self.privkey.as_bytes()
    }

    fn generate(&mut self, _rng: &mut dyn Random) {
        let (pubkey, privkey) = curve25519::keypair();
        self.pubkey = pubkey;
        self.privkey = privkey;
    }

    fn dh(&self, pubkey: &[u8], out: &mut [u8]) -> Result<(), ()> {
        let pubkey: [u8; curve25519::KEY_LEN] =
            pubkey[..32].try_into().expect("Wrong length on public key");
        let pubkey = curve25519::PublicKey::from(pubkey);
        let shared_secret = self.privkey.ecdh(&pubkey);
        out[..curve25519::KEY_LEN].copy_from_slice(&shared_secret.as_bytes());
        Ok(())
    }
}

pub struct HaclChachaPoly {
    key: [u8; chacha20poly1305::KEY_LEN],
}
impl Default for HaclChachaPoly {
    fn default() -> Self {
        skymd::skymd_init();
        Self {
            key: [0u8; chacha20poly1305::KEY_LEN],
        }
    }
}

impl Cipher for HaclChachaPoly {
    fn name(&self) -> &'static str {
        "ChaChaPoly"
    }

    fn set(&mut self, key: &[u8]) {
        self.key = key.try_into().expect("wrong size on key");
    }

    fn encrypt(&self, nonce: u64, authtext: &[u8], plaintext: &[u8], out: &mut [u8]) -> usize {
        let cipher_len = plaintext.len() + chacha20poly1305::TAG_LEN;
        assert!(out.len() >= cipher_len);

        let mut nonce_bytes = [0u8; chacha20poly1305::NONCE_LEN];
        nonce_bytes[16..].copy_from_slice(&nonce.to_le_bytes());

        let mut tag = [0u8; chacha20poly1305::TAG_LEN];

        chacha20poly1305::encrypt(&self.key, &nonce_bytes, authtext, plaintext, out, &mut tag);

        out[plaintext.len()..plaintext.len() + chacha20poly1305::TAG_LEN].copy_from_slice(&tag);
        cipher_len
    }

    fn decrypt(
        &self,
        nonce: u64,
        authtext: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<usize, ()> {
        let plaintext_len = ciphertext.len() - chacha20poly1305::TAG_LEN;
        assert!(out.len() >= plaintext_len);

        let mut nonce_bytes = [0u8; chacha20poly1305::NONCE_LEN];
        nonce_bytes[16..].copy_from_slice(&nonce.to_le_bytes());

        let tag: [u8; chacha20poly1305::TAG_LEN] = ciphertext
            [plaintext_len..plaintext_len + chacha20poly1305::TAG_LEN]
            .try_into()
            .expect("failed to get tag from ciphertext");

        let result = chacha20poly1305::decrypt(
            &self.key,
            &nonce_bytes,
            authtext,
            out,
            &ciphertext[..plaintext_len],
            &tag,
        );

        if result.is_ok() {
            Ok(plaintext_len)
        } else {
            Err(())
        }
    }
}
