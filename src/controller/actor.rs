use tokio::sync::{mpsc, oneshot};

use crate::{
    can::CanStats,
    caniot as ct,
    caniot::{self, build_telemetry_request, DeviceId, Endpoint, Response},
};
use serde::Serialize;

use super::{
    traits::ControllerAPI, Controller, ControllerError, ControllerStats, DeviceStats, GarageHandle, Device, DeviceTrait,
};

pub enum ControllerMessage {
    GetStats {
        respond_to: oneshot::Sender<(ControllerStats, Vec<DeviceStatsEntry>, CanStats)>,
    },
    Query {
        query: caniot::Request,
        timeout_ms: u32,
        respond_to: Option<oneshot::Sender<Result<caniot::Response, ControllerError>>>,
    },
}

#[derive(Debug, Clone)]
pub struct ControllerHandle {
    pub sender: mpsc::Sender<ControllerMessage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceStatsEntry {
    pub device_id_did: u8,
    pub device_id: caniot::DeviceId,
    pub stats: DeviceStats,
}

impl ControllerHandle {
    async fn execute<R>(&self, closure: impl FnOnce(oneshot::Sender<R>) -> ControllerMessage) -> R {
        let (sender, receiver) = oneshot::channel();
        let message = closure(sender);
        self.sender.send(message).await.unwrap();
        receiver.await.unwrap()
    }

    pub async fn get_stats(&self) -> (ControllerStats, Vec<DeviceStatsEntry>, CanStats) {
        self.execute(|respond_to| ControllerMessage::GetStats { respond_to })
            .await
    }

    // pub fn get_device(&self, did: DeviceId) -> DeviceHandle {
    //     DeviceHandle::new(did, self)
    // }

    pub fn get_garage_handle(&self) -> GarageHandle {
        GarageHandle::new(self)
    }

    pub fn get_device_handle<T>(&mut self, did: ct::DeviceId) -> DeviceHandle<T>
        where T: DeviceTrait
    {
        DeviceHandle::<T> { device: T::new(did), controller_handle: self}
    }
}

pub async fn handle_message(controller: &mut Controller, message: ControllerMessage) {
    match message {
        ControllerMessage::GetStats { respond_to } => {
            let _ = respond_to.send((
                controller.stats,
                controller.get_devices_stats(),
                controller.iface.stats,
            ));
        }
        ControllerMessage::Query {
            query,
            timeout_ms,
            respond_to,
        } => {
            if let Some(respond_to) = respond_to {
                controller.query_sched(query, timeout_ms, respond_to).await;
            } else {
                let _ = controller.send(query).await;
            }
        }
    }
}

#[async_trait]
impl ControllerAPI for ControllerHandle {
    async fn query(
        &mut self,
        frame: ct::Request,
        timeout_ms: u32,
    ) -> Result<ct::Response, ControllerError> {
        self.execute(|sender| ControllerMessage::Query {
            query: frame,
            timeout_ms,
            respond_to: Some(sender),
        })
        .await
    }

    async fn send(&mut self, frame: ct::Request) -> Result<(), ControllerError> {
        self.execute(|_sender| ControllerMessage::Query {
            query: frame,
            timeout_ms: 0,
            respond_to: None,
        })
        .await
    }
}


#[derive(Debug)]
pub struct DeviceHandle<'a, T>
    where T: DeviceTrait
{
    pub device: T,
    pub controller_handle: &'a mut ControllerHandle,
}

// #[derive(Debug, Clone)]
// pub struct DeviceHandle<'a, T> {
//     device: &'a mut Device<T>,
//     controller_handle: &'a ControllerHandle,
// }

// impl<'a> DeviceHandle<'a> {
//     fn new(did: DeviceId, controller_handle: &'a ControllerHandle) -> DeviceHandle {
//         DeviceHandle {
//             did,
//             controller_handle,
//         }
//     }

//     pub async fn request_telemetry(
//         &mutself,
//         endpoint: Endpoint,
//         timeout_ms: u32,
//     ) -> Result<Response, ControllerError> {
//         self.controller_handle
//             .query_telemetry(self.did, endpoint, timeout_ms)
//             .await
//     }
// }
