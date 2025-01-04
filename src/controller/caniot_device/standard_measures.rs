use chrono::NaiveTime;

use crate::{
    caniot::{traits::TempSensType, BoardClassTelemetry},
    utils::{monitorable_measure::ValueMonitor, Scheduling},
};

use super::JobTrait;

#[derive(Debug)]
pub struct DeviceMeasures {
    class_telemetry: Option<BoardClassTelemetry>,
    board_temp_monitor: ValueMonitor<f32>,
    outside_temp_monitor: ValueMonitor<f32>,
}

impl DeviceMeasures {
    pub fn update_class_telemetry(&mut self, telemetry: &BoardClassTelemetry) {
        self.class_telemetry = Some(*telemetry);

        if let Some(ref temp) = telemetry.get_temperature(TempSensType::BoardSensor) {
            self.board_temp_monitor.update(temp);
        }

        if let Some(ref temp) = telemetry.get_temperature(TempSensType::AnyExternal) {
            self.outside_temp_monitor.update(temp);
        }
    }

    pub fn reset_minmax(&mut self) {
        self.board_temp_monitor.reset();
        self.outside_temp_monitor.reset();
    }

    pub fn get_class_telemetry(&self) -> &Option<BoardClassTelemetry> {
        &self.class_telemetry
    }

    pub fn get_board_temp_monitor(&self) -> &ValueMonitor<f32> {
        &self.board_temp_monitor
    }

    pub fn get_outside_temp_monitor(&self) -> &ValueMonitor<f32> {
        &self.outside_temp_monitor
    }
}

impl Default for DeviceMeasures {
    fn default() -> Self {
        Self {
            class_telemetry: None,
            board_temp_monitor: ValueMonitor::new(),
            outside_temp_monitor: ValueMonitor::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceMeasuresResetJob(NaiveTime /* reset time */);

impl Default for DeviceMeasuresResetJob {
    fn default() -> Self {
        Self(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }
}

impl JobTrait for DeviceMeasuresResetJob {
    fn get_scheduling(&self) -> Scheduling {
        Scheduling::Daily(self.0)
    }
}
