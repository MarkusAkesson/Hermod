pub mod cli;
pub mod config;
pub mod consts;
pub mod genkey;
pub mod hermod;
pub mod host;
pub mod identity;
pub mod message;
pub mod noise;
pub mod peer;
pub mod request;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
