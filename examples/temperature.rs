use caniot_controller::caniot::Temperature;

fn main() {
    let temperature = Temperature::from_raw_u10(1);
    println!("Temperature: {:?}", temperature.to_celsius());
}
