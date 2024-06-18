use crate::caniot::{AsPayload, Cd, ClCd, Payload, ProtocolError, RequestData, SysCtrl};

use super::traits::{Class, ClassCommandTrait};

#[derive(Clone)]
pub struct BoardClassCommand<C: Class> {
    pub class_payload: C::Command,
    pub sys_ctrl: SysCtrl,
}

impl<C: Class> BoardClassCommand<C> {
    pub fn new(class_payload: Option<C::Command>, sys_ctrl: Option<SysCtrl>) -> Self {
        Self {
            class_payload: class_payload.unwrap_or_default(),
            sys_ctrl: sys_ctrl.unwrap_or_default(),
        }
    }

    pub fn into_request(self) -> RequestData {
        RequestData::Command {
            endpoint: crate::caniot::Endpoint::BoardControl,
            payload: self.into(),
        }
    }
}

impl<C: Class> TryFrom<&Payload<Cd>> for BoardClassCommand<C> {
    type Error = ProtocolError;

    fn try_from(value: &Payload<Cd>) -> Result<Self, Self::Error> {
        let sys_ctrl = if value.len() >= 8 {
            SysCtrl::from(value.data()[7])
        } else {
            SysCtrl::default()
        };

        Ok(Self {
            class_payload: C::Command::try_from(&Payload::new_unchecked(&value.data()[..7]))?,
            sys_ctrl,
        })
    }
}

impl<C: Class> Into<Payload<Cd>> for BoardClassCommand<C> {
    fn into(self) -> Payload<Cd> {
        let mut data = Vec::with_capacity(8);
        let pl = self.class_payload.to_payload();
        data.extend_from_slice(pl.as_ref());
        data.extend_from_slice(&[0_u8; 7][..7 - pl.len()]);
        data.extend_from_slice(&[self.sys_ctrl.into()]);

        Payload::new_from_vec(data).unwrap()
    }
}

// impl<C: Class> AsPayload<Cmd> for BoardClassCommand<C> {}
