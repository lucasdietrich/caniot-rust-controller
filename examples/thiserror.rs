use caniot_controller::controller::ControllerError;

fn main() {
    let err = ControllerError::UndifferentiablePendingQuery;
    let msg = format!("{}", err);

    println!("{}", msg);
}
