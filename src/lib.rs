pub mod cli;
pub mod client;
pub mod config;
pub mod consts;
pub mod genkey;
pub mod hermod;
pub mod host;
pub mod message;
pub mod noise;
pub mod peer;
pub mod request;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
