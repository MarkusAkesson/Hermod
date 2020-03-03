pub mod cli;
pub mod client;
pub mod config;
pub mod consts;
pub mod error;
pub mod genkey;
pub mod host;
pub mod identity;
pub mod log;
pub mod message;
pub mod noise;
pub mod peer;
pub mod request;
pub mod server;
pub mod share_key;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
