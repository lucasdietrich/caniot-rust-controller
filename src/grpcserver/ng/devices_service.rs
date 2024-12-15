use tonic::{Request, Response, Result, Status};

use crate::{
    caniot as ct,
    controller::{
        caniot_controller::auto_attach::{
            DEVICE_GARAGE_DID, DEVICE_HEATERS_DID, DEVICE_OUTDOOR_ALARM_DID,
        },
        DeviceAction, DeviceActionResult, DeviceInfos,
    },
    grpcserver::utc_to_prost_timestamp,
    shared::SharedHandle,
    utils::emulated_delay_async,
};

use super::model::{
    self as ng,
    devices::{
        self as m,
        caniot_devices_service_server::{CaniotDevicesService, CaniotDevicesServiceServer},
    },
};

#[derive(Debug)]
pub struct NgDevices {
    pub shared: SharedHandle,
}

impl NgDevices {
    async fn get_device_by_did(&self, did: ct::DeviceId) -> Result<Response<m::Device>, Status> {
        emulated_delay_async().await;
        if let Some(ref infos) = self
            .shared
            .controller_handle
            .get_caniot_device_infos(did)
            .await
        {
            Ok(Response::new(infos.into()))
        } else {
            Err(Status::not_found("Device not found"))
        }
    }
}

impl Into<m::Class0Telemetry> for ct::class0::Telemetry {
    fn into(self) -> m::Class0Telemetry {
        m::Class0Telemetry {
            in1: self.in1,
            in2: self.in2,
            in3: self.in3,
            in4: self.in4,
            oc1: self.oc1,
            oc2: self.oc2,
            rl1: self.rl1,
            rl2: self.rl2,
            int_temp: self.temp_in.to_celsius(),
            ext_temp0: self.temp_out[0].to_celsius(),
            ext_temp1: self.temp_out[1].to_celsius(),
            ext_temp2: self.temp_out[2].to_celsius(),
        }
    }
}

impl Into<m::Class1Telemetry> for ct::class1::Telemetry {
    fn into(self) -> m::Class1Telemetry {
        m::Class1Telemetry {
            ios: self.ios.to_vec(),
            int_temp: self.temp_in.to_celsius(),
            ext_temp0: self.temp_out[0].to_celsius(),
            ext_temp1: self.temp_out[1].to_celsius(),
            ext_temp2: self.temp_out[2].to_celsius(),
        }
    }
}

impl Into<m::device::Measures> for ct::classes::BoardClassTelemetry {
    fn into(self) -> m::device::Measures {
        match self {
            ct::BoardClassTelemetry::Class0(t) => m::device::Measures::Class0(t.into()),
            ct::BoardClassTelemetry::Class1(t) => m::device::Measures::Class1(t.into()),
        }
    }
}

impl Into<m::Device> for &DeviceInfos {
    fn into(self) -> m::Device {
        m::Device {
            did: Some(self.did.into()),
            is_seen: self.is_seen,
            last_seen: self.last_seen.as_ref().map(utc_to_prost_timestamp),
            last_seen_from_now: self.last_seen_from_now,
            controller_attached: self.controller_attached,
            controller_name: self.controller_display_name.clone(),
            stats: Some(m::DeviceStats {
                rx: self.stats.rx as u32,
                tx: self.stats.tx as u32,
                telemetry_rx: self.stats.telemetry_rx as u32,
                telemetry_tx: self.stats.telemetry_tx as u32,
                command_tx: self.stats.command_tx as u32,
                err_rx: self.stats.err_rx as u32,
                attribute_rx: self.stats.attribute_rx as u32,
                attribute_tx: self.stats.attribute_tx as u32,
                reset_requested: self.stats.reset_requested as u32,
                reset_settings_requested: self.stats.reset_settings_requested as u32,
                jobs_currently_scheduled: self.stats.jobs_currently_scheduled as u32,
                jobs_processed: self.stats.jobs_processed as u32,
            }),
            board_temp: self.board_temperature,
            outside_temp: self.outside_temperature,
            measures: self.measures.map(|m| m.into()),
            active_alert: self.active_alert.as_ref().map(|a| a.into()),
            ui_view_name: self.ui_view_name.clone(),
            ..Default::default()
        }
    }
}

#[tonic::async_trait]
impl CaniotDevicesService for NgDevices {
    async fn get_list(&self, _request: Request<()>) -> Result<Response<m::DevicesList>, Status> {
        let devices: Vec<m::Device> = self
            .shared
            .controller_handle
            .get_caniot_devices_infos_list()
            .await
            .iter()
            .map(|dev| dev.into())
            .collect();

        Ok(Response::new(m::DevicesList { devices }))
    }

    async fn get_devices_with_active_alert(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::DevicesList>, Status> {
        let devices: Vec<m::Device> = self
            .shared
            .controller_handle
            .get_caniot_devices_with_active_alert()
            .await
            .iter()
            .map(|dev| dev.into())
            .collect();

        Ok(Response::new(m::DevicesList { devices }))
    }

    async fn get(&self, request: Request<ng::DeviceId>) -> Result<Response<m::Device>, Status> {
        let did: ct::DeviceId = request.into_inner().into();
        if let Some(ref infos) = self
            .shared
            .controller_handle
            .get_caniot_device_infos(did)
            .await
        {
            Ok(Response::new(infos.into()))
        } else {
            Err(Status::not_found("Device not found"))
        }
    }

    async fn get_heaters_device(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::Device>, Status> {
        self.get_device_by_did(ct::DeviceId::from_u8(DEVICE_HEATERS_DID))
            .await
    }

    async fn get_garage_device(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::Device>, Status> {
        self.get_device_by_did(ct::DeviceId::from_u8(DEVICE_GARAGE_DID))
            .await
    }

    async fn get_outdoor_alarm_device(
        &self,
        _request: Request<()>,
    ) -> Result<Response<m::Device>, Status> {
        self.get_device_by_did(ct::DeviceId::from_u8(DEVICE_OUTDOOR_ALARM_DID))
            .await
    }

    async fn perform_action(
        &self,
        request: tonic::Request<m::Action>,
    ) -> std::result::Result<tonic::Response<m::ActionResult>, tonic::Status> {
        let action = request.into_inner();

        let did: ct::DeviceId = action
            .did
            .ok_or_else(|| Status::invalid_argument("Missing did or action"))?
            .into();

        let action = match action
            .action
            .ok_or(Status::invalid_argument("Missing did or action"))?
        {
            m::action::Action::Reboot(..) => DeviceAction::Reset,
            m::action::Action::ResetSettings(..) => DeviceAction::ResetSettings,
            m::action::Action::Inhibit(inhibit) => {
                let inhibit = ng::TwoStatePulse::try_from(inhibit)
                    .map_err(|e| Status::invalid_argument(format!("Invalid inhibit: {:?}", e)))?;
                DeviceAction::InhibitControl(inhibit.into())
            }
            m::action::Action::Ping(endpoint) => {
                let endpoint = ng::Endpoint::try_from(endpoint)
                    .map_err(|e| Status::invalid_argument(format!("Invalid endpoint: {:?}", e)))?;
                DeviceAction::Ping(endpoint.into())
            }
        };

        // TODO is it important to compare the result type with the action type to verify they match?
        // should it be done in the controller?
        let result = self
            .shared
            .controller_handle
            .caniot_device_action(Some(did), action, None)
            .await
            .map_err(|e| Status::internal(format!("Error in perform_action: {} ({:?})", e, e)))?;

        let result = match result {
            DeviceActionResult::ResetSent => m::action_result::ActionResult::Reboot(()),
            DeviceActionResult::ResetSettingsSent => {
                m::action_result::ActionResult::ResetSettings(())
            }
            DeviceActionResult::InhibitControlSent => {
                m::action_result::ActionResult::Inhibit(true) // change
            }
            DeviceActionResult::Pong(response) => {
                m::action_result::ActionResult::Pong(response.into())
            }
            _ => {
                return Err(Status::internal("Invalid action result"));
            }
        };

        let infos = &self
            .shared
            .controller_handle
            .get_caniot_device_infos(did)
            .await
            .ok_or(Status::not_found("Device not found"))?;

        Ok(Response::new(m::ActionResult {
            device: Some(infos.into()),
            action_result: Some(result),
        }))
    }
}

pub fn get_ng_devices_server(shared: SharedHandle) -> CaniotDevicesServiceServer<NgDevices> {
    CaniotDevicesServiceServer::new(NgDevices { shared })
}
