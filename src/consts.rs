pub const PACKET_MAXLENGTH: usize = 65536;
pub const MAC_LENGTH: usize = 16;

pub const MSG_HEADER_LEN: usize = MSG_TYPE_LEN + MSG_LENGTH_LEN;
pub const MSG_TYPE_LEN: usize = 1;
pub const MSG_LENGTH_LEN: usize = 2;

pub static NOISE_PATTERN: &'static str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";

pub const SERVER_PRIVATE_KEY_FILE: &'static str = ".hermod/server_key";
pub const SERVER_PUBLIC_KEY_FILE: &'static str = ".hermod/server_key.pub";

pub const HERMOD_HS_INIT_LEN: usize = 100;
pub const HERMOD_HS_RESP_LEN: usize = 100;
pub const HERMOD_REQ_MSG_LEN: usize = 100;
pub const HERMOD_PAT_MSG_LEN: usize = 100;
pub const HERMOD_ERR_MSG_LEN: usize = 100;
