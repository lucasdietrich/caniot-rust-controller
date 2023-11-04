use tokio::{sync::{mpsc, oneshot}, runtime::Runtime, task::JoinHandle};

use crate::can::CanStats;

use super::{Controller, CaniotStats};

const CHANNEL_SIZE: usize = 10;

enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(CaniotStats, CanStats)>,
    },
    Query {
        respond_to: oneshot::Sender<()>,
    },
}

pub struct ControllerActor {
    controller: Controller,
    receiver: mpsc::Receiver<ControllerMessage>,
}

impl ControllerActor {
    fn new(controller: Controller, receiver: mpsc::Receiver<ControllerMessage>) -> Self {
        ControllerActor { controller, receiver }
    }

    async fn handle_message(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::GetStats { respond_to } => {
                let _ =respond_to.send((self.controller.stats, self.controller.iface.stats));
            },
            ControllerMessage::Query { respond_to } => {
                let _ = respond_to.send(());
            }
        }
    }

    async fn run(mut self) {
        // self.controller.run().await;

        while let Some(message) = self.receiver.recv().await {
            self.handle_message(message).await;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ControllerActorHandle {
    sender: mpsc::Sender<ControllerMessage>,
}

impl ControllerActorHandle {
    pub fn new(rt: &Runtime, controller: Controller) -> Self {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);
        let actor = ControllerActor::new(controller, receiver);
        rt.spawn(actor.run());
        Self { sender }
    }

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