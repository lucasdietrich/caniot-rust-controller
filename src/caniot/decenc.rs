use rocket::time::format_description::modifier::End;

// decode/encode
use crate::caniot::protocol::Endpoint;
use crate::caniot::types::*;

pub fn decode_cls0_telemetry(payload: &[u8]) -> Option<Class0Payload> {
    Some(Class0Payload {
        oc1: payload[0] & 0b0000_0001 != 0,
        oc2: payload[0] & 0b0000_0010 != 0,
        rl1: payload[0] & 0b0000_0100 != 0,
        rl2: payload[0] & 0b0000_1000 != 0,
        in1: payload[0] & 0b0001_0000 != 0,
        in2: payload[0] & 0b0010_0000 != 0,
        in3: payload[0] & 0b0100_0000 != 0,
        in4: payload[0] & 0b1000_0000 != 0,
        poc1: payload[1] & 0b0000_0001 != 0,
        puc2: payload[1] & 0b0000_0010 != 0,
        prl1: payload[1] & 0b0000_0100 != 0,
        prl2: payload[1] & 0b0000_1000 != 0,
        temp_in: i16::from_le_bytes([payload[2], payload[3] & 0b0000_0011]),
        temp_out: [
            i16::from_le_bytes([payload[3] >> 2, payload[4] & 0b0000_1111]),
            i16::from_le_bytes([payload[4] >> 4, payload[5] & 0b0000_1111]),
            i16::from_le_bytes([payload[5] >> 6, payload[6] & 0b0000_0011]),
        ],
    })
}

pub fn encode_sys_command(sysc: SystemCommand) -> u8 {
    let mut payload = 0_u8;

    payload |= sysc.hardware_reset as u8;
    payload |= (sysc.software_reset as u8) << 1;
    payload |= (sysc.watchdog_reset as u8) << 2;
    payload |= (sysc.watchdog_enable as u8) << 3;
    payload |= (sysc.factory_reset as u8) << 5;

    payload
}

pub fn encode_cls0_command(command: Class0Command) -> [u8; 8] {
    let mut payload = [0_u8; 8];

    payload[0] = command.coc1 as u8;
    payload[0] |= (command.coc2 as u8) << 3;
    payload[0] |= ((command.crl1 as u8) & 0b11) << 6;
    payload[1] = ((command.crl1 as u8) & 0b100) >> 2;
    payload[1] |= (command.crl2 as u8) << 1;
    payload[7] = encode_sys_command(command.sys);

    payload
}

// pub fn interpret_telemetry_payload(endpoint: Endpoint, payload: &[u8]) {

// }
