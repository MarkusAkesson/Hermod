pub enum HermodError {
    FileNotFound,
    OpeningFile,
    CreatingFile,
    ConnectionRefused,
    Encryption,
    Decryption,
    Handshake,
    UnknownMessage,
    UnknownIdentity,
    UnknownHost,
    UnauthorizedIdentity,
}
