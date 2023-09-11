use socketcan::{CanSocket, Frame, Socket};

fn main() {
    let iface = "can0";

    let sock = CanSocket::open(iface).unwrap();

    loop {
        let frame = sock.read_frame().unwrap();
        println!("{}  {}", iface, frame_to_string(&frame));
    }
}

fn frame_to_string<F: Frame>(frame: &F) -> String {
    let id = frame.raw_id();
    let data_string = frame
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));

    format!("{:X}  [{}] {}", id, frame.dlc(), data_string)
}