use crate::coprocessor::{CoproHandle, CoproMessage};

use thiserror::Error;

pub struct CoproController {
    handle: CoproHandle,
}

#[derive(Debug, Error)]
pub enum CoproError {}

impl CoproController {
    pub fn new(handle: CoproHandle) -> Result<CoproController, CoproError> {
        Ok(CoproController { handle })
    }

    pub async fn poll_message(&mut self) -> Option<CoproMessage> {
        self.handle.receiver.recv().await
    }

    pub async fn handle_message(&mut self, message: CoproMessage) {
        match message {
            CoproMessage::XiaomiRecord(_record) => {
                // Do something
            }
        }
    }
}
