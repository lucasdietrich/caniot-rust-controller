use crate::controller::actor::ControllerMessage;

use super::{traits::ControllerAPI, Controller};

impl Controller {
    pub async fn handle_api_message(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::GetStats { respond_to } => {
                let _ =
                    respond_to.send((self.stats, self.get_devices_stats(), self.iface.get_stats()));
            }
            ControllerMessage::Query {
                query,
                timeout_ms,
                respond_to,
            } => {
                if let Some(respond_to) = respond_to {
                    self.query_sched(query, timeout_ms, respond_to).await;
                } else {
                    let _ = self.send(query).await;
                }
            }
            ControllerMessage::DeviceAction {
                did: _,
                action,
                respond_to: _,
            } => {
                todo!("DeviceAction: {action:?}");
            }
        }
    }
}
