use bytes::Buf;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ruuvi5 {
    data_format: u8,
    temperature: Option<f32>,
    humidity: Option<f32>,
    pressure: Option<u32>,
    acceleration_x: Option<f32>,
    acceleration_y: Option<f32>,
    acceleration_z: Option<f32>,
    voltage: Option<f32>,
    tx_power: Option<i8>,
    movement_counter: Option<u8>,
    measurement_sequence: Option<u16>,
    mac: Option<String>,
}

impl Ruuvi5 {
    pub fn new(input: &[u8]) -> Self {
        let mut buf = input;

        Self {
            data_format: buf.get_u8(),

            temperature: Some(buf.get_i16())
                .filter(|n| n > &i16::MIN)
                .map(|n| f32::from(n) * 0.005)
                .map(round),

            humidity: Some(buf.get_u16())
                .filter(|n| n <= &40000)
                .map(|n| f32::from(n) * 0.0025)
                .map(round),

            pressure: Some(buf.get_u16())
                .filter(|n| n < &u16::MAX)
                .map(|n| u32::from(n) + 50000),

            acceleration_x: Some(buf.get_i16())
                .filter(|n| n > &i16::MIN)
                .map(|n| f32::from(n) * 0.001)
                .map(round),

            acceleration_y: Some(buf.get_i16())
                .filter(|n| n > &i16::MIN)
                .map(|n| f32::from(n) * 0.001)
                .map(round),

            acceleration_z: Some(buf.get_i16())
                .filter(|n| n > &i16::MIN)
                .map(|n| f32::from(n) * 0.001)
                .map(round),

            voltage: Some(buf.get_u16() >> 5)
                .filter(|n| n <= &2046)
                .map(|n| f32::from(n) * 0.001 + 1.6)
                .map(round),

            tx_power: Some(input[14] & 0x1f)
                .filter(|n| n <= &30)
                .map(|n| i8::try_from(n).unwrap() * 2 - 40),

            movement_counter: Some(buf.get_u8()).filter(|n| n < &u8::MAX),

            measurement_sequence: Some(buf.get_u16()).filter(|n| n < &u16::MAX),

            mac: Some(format!(
                "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                buf.get_u8(),
                buf.get_u8(),
                buf.get_u8(),
                buf.get_u8(),
                buf.get_u8(),
                buf.get_u8()
            ))
            .filter(|s| s != "FF:FF:FF:FF:FF:FF"),
        }
    }
}

fn round(n: f32) -> f32 {
    (n * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::Ruuvi5;

    const VALID_INPUT: &[u8; 24] = b"\x05\x12\xFC\x53\x94\xC3\x7C\x00\x04\xFF\xFC\x04\
                                     \x0C\xAC\x36\x42\x00\xCD\xCB\xB8\x33\x4C\x88\x4F";

    const MAX_INPUT: &[u8; 24] = b"\x05\x7F\xFF\x9c\x40\xFF\xFE\x7F\xFF\x7F\xFF\x7F\
                                   \xFF\xFF\xDE\xFE\xFF\xFE\xCB\xB8\x33\x4C\x88\x50";

    const MIN_INPUT: &[u8; 24] = b"\x05\x80\x01\x00\x00\x00\x00\x80\x01\x80\x01\x80\
                                   \x01\x00\x00\x00\x00\x00\xCB\xB8\x33\x4C\x88\x51";

    const INVALID_INPUT: &[u8; 24] = b"\x05\x80\x00\xFF\xFF\xFF\xFF\x80\x00\x80\x00\x80\
                                       \x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF";

    fn valid_output() -> Ruuvi5 {
        Ruuvi5 {
            data_format: 5,
            temperature: Some(24.3),
            humidity: Some(53.49),
            pressure: Some(100044),
            acceleration_x: Some(0.004),
            acceleration_y: Some(-0.004),
            acceleration_z: Some(1.036),
            voltage: Some(2.977),
            tx_power: Some(4),
            movement_counter: Some(66),
            measurement_sequence: Some(205),
            mac: Some("CB:B8:33:4C:88:4F".to_string()),
        }
    }

    fn max_output() -> Ruuvi5 {
        Ruuvi5 {
            data_format: 5,
            temperature: Some(163.835),
            humidity: Some(100.0),
            pressure: Some(115534),
            acceleration_x: Some(32.767),
            acceleration_y: Some(32.767),
            acceleration_z: Some(32.767),
            voltage: Some(3.646),
            tx_power: Some(20),
            movement_counter: Some(254),
            measurement_sequence: Some(65534),
            mac: Some("CB:B8:33:4C:88:50".to_string()),
        }
    }

    fn min_output() -> Ruuvi5 {
        Ruuvi5 {
            data_format: 5,
            temperature: Some(-163.835),
            humidity: Some(0.0),
            pressure: Some(50000),
            acceleration_x: Some(-32.767),
            acceleration_y: Some(-32.767),
            acceleration_z: Some(-32.767),
            voltage: Some(1.6),
            tx_power: Some(-40),
            movement_counter: Some(0),
            measurement_sequence: Some(0),
            mac: Some("CB:B8:33:4C:88:51".to_string()),
        }
    }

    fn invalid_output() -> Ruuvi5 {
        Ruuvi5 {
            data_format: 5,
            temperature: None,
            humidity: None,
            pressure: None,
            acceleration_x: None,
            acceleration_y: None,
            acceleration_z: None,
            voltage: None,
            tx_power: None,
            movement_counter: None,
            measurement_sequence: None,
            mac: None,
        }
    }

    #[test]
    fn valid() {
        let data = Ruuvi5::new(&VALID_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi5 = serde_json::from_str(&json).unwrap();
        assert_eq!(&valid_output(), &output);
    }

    #[test]
    fn maximum() {
        let data = Ruuvi5::new(&MAX_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi5 = serde_json::from_str(&json).unwrap();
        assert_eq!(&max_output(), &output);
    }

    #[test]
    fn minimum() {
        let data = Ruuvi5::new(&MIN_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi5 = serde_json::from_str(&json).unwrap();
        assert_eq!(&min_output(), &output);
    }

    #[test]
    fn invalid() {
        let data = Ruuvi5::new(&INVALID_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi5 = serde_json::from_str(&json).unwrap();
        assert_eq!(&invalid_output(), &output);
    }
}
