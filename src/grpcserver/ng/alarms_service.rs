use chrono::{DateTime, Utc};
use tonic::{Request, Response, Result, Status};

use crate::{
    controller::{
        alarms, Action, AlarmControllerReport, AlarmEnable, LightAction, LightsActions, SirenAction,
    },
    grpcserver::{naive_time_to_string, string_to_naive_time, utc_to_prost_timestamp},
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

impl Into<m::AlarmConfig> for &alarms::AlarmConfig {
    fn into(self) -> m::AlarmConfig {
        m::AlarmConfig {
            alarm_auto_enabled: self.auto_alarm_enable,
            alarm_auto_enable_time: naive_time_to_string(&self.auto_alarm_enable_time),
            alarm_auto_disable_time: naive_time_to_string(&self.auto_alarm_disable_time),
            lights_auto_enabled: self.auto_lights_enable,
            alarm_siren_minimum_interval_seconds: self.alarm_siren_minimum_interval_seconds,
            lights_auto_enable_time: naive_time_to_string(&self.auto_lights_enable_time),
            lights_auto_disable_time: naive_time_to_string(&self.auto_lights_disable_time),
        }
    }
}

impl Into<m::AlarmConfig> for alarms::AlarmConfig {
    fn into(self) -> m::AlarmConfig {
        (&self).into()
    }
}

impl Into<alarms::AlarmPartialConfig> for m::AlarmPartialConfig {
    fn into(self) -> alarms::AlarmPartialConfig {
        alarms::AlarmPartialConfig {
            auto_alarm_enable: self.alarm_auto_enabled,
            auto_alarm_enable_time: self
                .alarm_auto_enable_time
                .map(|t| string_to_naive_time(&t)),
            auto_alarm_disable_time: self
                .alarm_auto_disable_time
                .map(|t| string_to_naive_time(&t)),
            alarm_siren_minimum_interval_seconds: self.alarm_siren_minimum_interval_seconds,
            auto_lights_enable: self.lights_auto_enabled,
            auto_lights_enable_time: self
                .lights_auto_enable_time
                .map(|t| string_to_naive_time(&t)),
            auto_lights_disable_time: self
                .lights_auto_disable_time
                .map(|t| string_to_naive_time(&t)),
            // detection_time_ranges: None, // TODO
        }
    }
}

impl NgAlarms {
    fn alarms_state_to_proto(
        &self,
        state: &AlarmControllerReport,
        now: &DateTime<Utc>,
    ) -> m::OutdoorAlarmState {
        m::OutdoorAlarmState {
            device: Some(m::OutdoorAlarmDeviceState {
                south_light: state.ios.get_south_light(),
                east_light: state.ios.get_east_light(),
                east_detector: state.ios.get_east_detector(),
                south_detector: state.ios.get_south_detector(),
                siren_active: state.ios.siren,
                sabotage: state.ios.get_sabotage(),
            }),
            enabled: state.alarm_enabled,
            last_siren: state
                .last_siren_activation
                .map(|dt| utc_to_prost_timestamp(&dt)),
            sirens_triggered_count: state.stats.sirens_triggered_count,
            south_detector_triggered_count: state.stats.south_detector_triggered_count,
            east_detector_triggered_count: state.stats.east_detector_triggered_count,
            sabotage_triggered_count: state.stats.sabotage_triggered_count,
            signals_total_count: state.stats.south_detector_triggered_count
                + state.stats.east_detector_triggered_count
                + state.stats.sabotage_triggered_count,
            last_signal_from_now_seconds: state
                .stats
                .last_event
                .map(|dt| (*now - dt).num_seconds() as u32),
            last_signal: state.stats.last_event.map(|dt| utc_to_prost_timestamp(&dt)),
            config: Some((&state.config).into()),
            last_siren_from_now_seconds: state
                .last_siren_activation
                .map(|dt| (*now - dt).num_seconds() as u32),
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

        Ok(Response::new(
            self.alarms_state_to_proto(&result, &Utc::now()),
        ))
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
            Ok(Response::new(
                self.alarms_state_to_proto(&result, &Utc::now()),
            ))
        } else {
            self.get_outdoor_alarm_state_inner().await
        }
    }

    async fn get_config(
        &self,
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<m::AlarmConfig>, tonic::Status> {
        let action = Action::GetConfig;
        let result = self
            .shared
            .controller_handle
            .device_action_inner(None, action, None)
            .await
            .map_err(|e| Status::internal(format!("Error in get_config: {} ({:?})", e, e)))?;

        Ok(Response::new(result.config.into()))
    }

    async fn set_config(
        &self,
        request: tonic::Request<m::AlarmPartialConfig>,
    ) -> std::result::Result<tonic::Response<m::AlarmConfig>, tonic::Status> {
        let action = Action::SetConfig(request.into_inner().into());
        let result = self
            .shared
            .controller_handle
            .device_action_inner(None, action, None)
            .await
            .map_err(|e| Status::internal(format!("Error in set_config: {} ({:?})", e, e)))?;

        Ok(Response::new(result.config.into()))
    }
}

pub fn get_ng_alarms_server(shared: SharedHandle) -> AlarmsServiceServer<NgAlarms> {
    AlarmsServiceServer::new(NgAlarms { shared })
}
