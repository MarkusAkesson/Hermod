use crate::config::ClientConfig;
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RequestMethod {
    Upload = 1,
    Download,
}

pub struct Request<'a> {
    source: &'a str,
    destination: &'a str,
    method: RequestMethod,
}

impl<'a> Request<'a> {
    pub fn new(config: &'a ClientConfig) -> Self {
        Request {
            source: &config.source,
            destination: &config.destination,
            method: config.request,
        }
    }
}
