use caniot_controller::caniot::{
    class0::{self, Class0},
    BoardClassCommand, DeviceId, Frame, RequestData, SysCtrl, Xps,
};

fn main() {
    let class_command = class0::Command {
        crl1: Xps::PulseOn,
        ..Default::default()
    };

    let sys = SysCtrl::HARDWARE_RESET;

    let blc = BoardClassCommand::<Class0>::new(Some(class_command), Some(sys));
    let req = blc.into_request();

    let did = DeviceId::from_u8(1);
    let frame = Frame::<RequestData>::new(did, req);

    println!(
        "id: {:?} pl: {:?}",
        frame.get_can_id(),
        frame.get_can_payload()
    );
}
