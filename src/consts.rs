pub const PACKET_MAXLENGTH: usize = 65536;
pub const MAC_LENGTH: usize = 16;

pub const HERMOD_PORT: u16 = 4444;

pub const MSG_HEADER_LEN: usize = MSG_TYPE_LEN + MSG_LENGTH_LEN;
pub const MSG_TYPE_LEN: usize = 1;
pub const MSG_LENGTH_LEN: usize = 4;
pub const MSG_PAYLOAD_LEN: usize = PACKET_MAXLENGTH - MSG_HEADER_LEN - MAC_LENGTH;

pub const AEAD_TAG_LEN: usize = 16;
pub const REKEY_THRESHOLD: usize = 1073741824; // 1 GB

pub const ID_TOKEN_LEN: u8 = 8;
pub const ID_TOKEN_B64LEN: u8 = 12;

pub const NOISE_PATTERN: &'static str = "Noise_KK_25519_ChaChaPoly_BLAKE2s";
pub const SHARE_KEY_PATTERN: &'static str = "Noise_XX_25519_ChaChaPoly_BLAKE2s";

pub const HERMOD_BASE_DIR: &'static str = ".hermod";

pub const SERVER_PRIVATE_KEY_FILE: &'static str = "server_key";
pub const SERVER_PUBLIC_KEY_FILE: &'static str = "server_key.pub";

pub const HERMOD_LOG_FILE: &'static str = "server.log";

pub const HERMOD_HS_INIT_LEN: usize = 48;
pub const HERMOD_HS_RESP_LEN: usize = 48;

pub const HERMOD_KS_INIT_LEN: usize = 32;
pub const HERMOD_KS_RES1_LEN: usize = 96 + MSG_TYPE_LEN;
pub const HERMOD_KS_RES2_LEN: usize = 76 + MSG_TYPE_LEN;
