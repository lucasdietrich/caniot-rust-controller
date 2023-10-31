use tokio::{sync::{mpsc, oneshot}, runtime::Runtime, task::JoinHandle};

use super::Controller;

const CHANNEL_SIZE: usize = 10;

#[derive(Debug)]
pub struct ControllerResponse {

}

#[derive(Debug)]
pub struct ControllerMessage {
    respond_to: oneshot::Sender<ControllerResponse>,
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
        println!("ControllerMessage {:?}", message);
        let ControllerMessage { respond_to } = message;
        let _ = respond_to.send(ControllerResponse {});
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
        let msg = ControllerMessage { respond_to };
        self.sender.send(msg).await.unwrap();
        println!("query response received");
        let _ = recv.await.unwrap();
        Ok(())
    }
}