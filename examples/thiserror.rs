use caniot_controller::controller::caniot_controller::caniot_devices_controller::CaniotControllerError;

fn main() {
    let err = CaniotControllerError::UndifferentiablePendingQuery;
    let msg = format!("{}", err);

    println!("{}", msg);
}
