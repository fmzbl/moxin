mod backend_impls;
mod fake_data;

use moxin_protocol::protocol::Command;
use std::sync::mpsc;

pub struct Backend {
    pub command_sender: mpsc::Sender<Command>,
}

impl Default for Backend {
    fn default() -> Self {
        Backend::new()
    }
}

impl Backend {
    pub fn new() -> Backend {
        let home = std::env::var("HOME").unwrap();

        let command_sender = backend_impls::BackendImpl::new(format!("{home}/ai"));
        Backend { command_sender }
    }
}
