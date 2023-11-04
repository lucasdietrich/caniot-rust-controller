use tokio::{sync::{mpsc, oneshot}, runtime::Runtime, task::JoinHandle};

use crate::can::CanStats;

use super::{Controller, CaniotStats};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(CaniotStats, CanStats)>,
    },
    Query {
        respond_to: oneshot::Sender<()>,
    },
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    pub sender: mpsc::Sender<ControllerMessage>,
}

impl ControllerHandle {
    pub async fn query(&self) -> Result<(), ()> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::Query { respond_to };
        self.sender.send(msg).await.unwrap();
        recv.await.map_err(|_| ())
    }

    pub async fn get_stats(&self) -> Result<(CaniotStats, CanStats), ()> {
        let (respond_to, recv) = oneshot::channel();
        let msg = ControllerMessage::GetStats { respond_to };
        self.sender.send(msg).await.unwrap();
        recv.await.map_err(|_| ())
    }
}