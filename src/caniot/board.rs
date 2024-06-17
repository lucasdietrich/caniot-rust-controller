// use super::{traits::Class, Endpoint, RequestData, SysCtrl};

// /// Board Level Controller (BLC) Class Command
// pub struct BoardClassCommand<C: Class> {
//     pub class_payload: <C as Class>::Command,
//     pub sys: SysCtrl,
// }

// impl<C> BoardClassCommand<C>
// where
//     C: Class,
// {
//     pub fn to_request(self) -> RequestData {
//         RequestData::Command {
//             endpoint: Endpoint::BoardControl,
//             payload: {
//                 let mut vec: Vec<_> = self.class_payload.into();
//                 vec.push(self.sys.into());
//                 vec
//             },
//         }
//     }

//     pub fn hardware_reset() -> BoardClassCommand<C> {
//         BoardClassCommand {
//             class_payload: <C as Class>::Command::default(),
//             sys: SysCtrl::HARDWARE_RESET,
//         }
//     }
// }

// #[cfg(tests)]
// mod tests {
//     use super::*;
// }
