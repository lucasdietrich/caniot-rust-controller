use crate::caniot::{self, class0, traits::ClassCommandTrait, Xps};

#[derive(Default)]
pub struct OutdoorAlarmCommand(pub class0::Command);

impl OutdoorAlarmCommand {
    pub fn new(south: Xps, east: Xps, siren: Xps) -> Self {
        OutdoorAlarmCommand(class0::Command {
            coc1: south,
            coc2: east,
            crl1: siren,
            crl2: Xps::None,
        })
    }

    pub fn set_siren(&mut self, cmd: Xps) {
        self.0.crl1 = cmd;
    }

    pub fn set_east_light(&mut self, cmd: Xps) {
        self.0.coc1 = cmd;
    }

    pub fn set_south_light(&mut self, cmd: Xps) {
        self.0.coc2 = cmd;
    }

    pub fn into_request(self) -> caniot::RequestData {
        self.0.to_request()
    }
}
