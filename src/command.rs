use crate::config::ClientConfig;
use crate::request::Request;

use async_std::sync::{Receiver, Sender};

pub struct Command<'a> {
    source: &'a str,
    destination: &'a str,
    method: Request,
    rx: Receiver<()>,
}

impl<'a> Command<'a> {
    pub fn new(config: &'a ClientConfig, rx: Receiver<()>) -> Self {
        Command {
            source: &config.source,
            destination: &config.destination,
            method: config.request,
            rx: rx,
        }
    }
}
