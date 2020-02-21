pub const PACKET_MAXLENGTH: usize = 65536;
pub const MAC_LENGTH: usize = 16;

pub const MSG_HEADER_LEN: usize = MSG_TYPE_LEN + MSG_LENGTH_LEN;
pub const MSG_TYPE_LEN: usize = 1;
pub const MSG_LENGTH_LEN: usize = 4;
pub const MSG_PAYLOAD_LEN: usize = PACKET_MAXLENGTH - MSG_HEADER_LEN - MAC_LENGTH;

pub const ID_TOKEN_LEN: u8 = 8;

pub const NOISE_PATTERN: &'static str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";

pub const SERVER_PRIVATE_KEY_FILE: &'static str = ".hermod/server_key";
pub const SERVER_PUBLIC_KEY_FILE: &'static str = ".hermod/server_key.pub";

pub const HERMOD_HS_INIT_LEN: usize = 32;
pub const HERMOD_HS_RESP_LEN: usize = 48;
