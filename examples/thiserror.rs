use caniot_controller::controller::caniot_controller::caniot_devices_controller::ControllerError;

fn main() {
    let err = ControllerError::UndifferentiablePendingQuery;
    let msg = format!("{}", err);

    println!("{}", msg);
}
