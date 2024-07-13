use tonic::{Request, Response, Result, Status};

use crate::{
    controller::{
        Action, AlarmControllerReport, AlarmEnable, LightAction, LightsActions, SirenAction,
    },
    grpcserver::{naive_time_to_string, utc_to_prost_timestamp},
    shared::SharedHandle,
};

use super::model::alarms::{
    self as m,
    alarms_service_server::{AlarmsService, AlarmsServiceServer},
};

#[derive(Debug)]
pub struct NgAlarms {
    pub shared: SharedHandle,
}

impl Into<LightAction> for m::TwoStates {
    fn into(self) -> LightAction {
        match self {
            m::TwoStates::None => LightAction::None,
            m::TwoStates::Off => LightAction::Off,
            m::TwoStates::On => LightAction::On,
            m::TwoStates::Toggle => LightAction::Toggle,
        }
    }
}

impl NgAlarms {
    fn alarms_state_to_proto(&self, state: &AlarmControllerReport) -> m::OutdoorAlarmState {
        m::OutdoorAlarmState {
            device: Some(m::OutdoorAlarmDeviceState {
                east_light: state.ios.lights[0],
                south_light: state.ios.lights[1],
                east_detector: state.ios.detectors[0],
                south_detector: state.ios.detectors[1],
                siren_active: state.ios.siren,
                sabotage: state.ios.sabotage,
            }),
            enabled: state.alarm_enabled,
            last_siren: state
                .last_siren_activation
                .map(|dt| utc_to_prost_timestamp(&dt)),
            siren_triggered_count: state.siren_triggered_count,
            alarm_auto_enabled: state.config.auto_alarm_enable,
            alarm_auto_enable_time: naive_time_to_string(&state.config.auto_alarm_enable_time),
            alarm_auto_disable_time: naive_time_to_string(&state.config.auto_alarm_disable_time),
            lights_auto_enabled: state.config.auto_lights_enable,
            alarm_siren_minimum_interval_seconds: state.config.alarm_siren_minimum_interval_seconds,
            lights_auto_enable_time: naive_time_to_string(&state.config.auto_lights_enable_time),
            lights_auto_disable_time: naive_time_to_string(&state.config.auto_lights_disable_time),
            ..Default::default()
        }
    }

    async fn get_outdoor_alarm_state_inner(
        &self,
    ) -> Result<Response<m::OutdoorAlarmState>, Status> {
        let action = Action::GetStatus;
        let result = self
            .shared
            .controller_handle
            .device_action_inner(None, action, None)
            .await
            .map_err(|e| {
                Status::internal(format!("Error in get_outdoor_alarm_state: {} ({:?})", e, e))
            })?;

        Ok(Response::new(self.alarms_state_to_proto(&result)))
    }
}

#[tonic::async_trait]
impl AlarmsService for NgAlarms {
    async fn get_outdoor_alarm_state(
        &self,
        _req: Request<()>,
    ) -> Result<Response<m::OutdoorAlarmState>, Status> {
        self.get_outdoor_alarm_state_inner().await
    }

    async fn send_outdoor_alarm_command(
        &self,
        req: Request<m::OutdoorAlarmCommand>,
    ) -> Result<Response<m::OutdoorAlarmState>, Status> {
        let action = match req
            .into_inner()
            .inner
            .expect("Missing OutdoorAlarmState inner command")
        {
            m::outdoor_alarm_command::Inner::Lights(lights) => {
                println!("Lights: {:?}", lights);
                let east = m::TwoStates::try_from(lights.east_light)
                    .unwrap_or_default()
                    .into();
                let south = m::TwoStates::try_from(lights.south_light)
                    .unwrap_or_default()
                    .into();
                Some(Action::SetLights(LightsActions { east, south }))
            }
            m::outdoor_alarm_command::Inner::OutdoorAlarmEnable(ts) => {
                let ts = m::TwoStates::try_from(ts).unwrap_or_default();
                match ts {
                    m::TwoStates::None => None,
                    m::TwoStates::On => Some(Action::SetAlarm(AlarmEnable::Armed)),
                    m::TwoStates::Off => Some(Action::SetAlarm(AlarmEnable::Disarmed)),
                    m::TwoStates::Toggle => {
                        return Err(Status::invalid_argument("Invalid alarm state"))
                    }
                }
            }
            m::outdoor_alarm_command::Inner::OutdoorAlarmSirenDirectAction(sa) => {
                let sa = m::SirenAction::try_from(sa).unwrap_or_default();
                match sa {
                    m::SirenAction::ForceOff => Some(Action::SirenAction(SirenAction::ForceOff)),
                }
            }
        };

        if let Some(action) = action {
            let result = self
                .shared
                .controller_handle
                .device_action_inner(None, action, None)
                .await
                .map_err(|e| {
                    Status::internal(format!(
                        "Error in send_outdoor_alarm_command: {} ({:?})",
                        e, e
                    ))
                })?;
            Ok(Response::new(self.alarms_state_to_proto(&result)))
        } else {
            self.get_outdoor_alarm_state_inner().await
        }
    }
}

pub fn get_ng_alarms_server(shared: SharedHandle) -> AlarmsServiceServer<NgAlarms> {
    AlarmsServiceServer::new(NgAlarms { shared })
}
