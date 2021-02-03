use bytes::Buf;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ruuvi3 {
    data_format: u8,
    humidity: Option<f32>,
    temperature: Option<f32>,
    pressure: Option<u32>,
    acceleration_x: Option<f32>,
    acceleration_y: Option<f32>,
    acceleration_z: Option<f32>,
    voltage: Option<f32>,
}

impl Ruuvi3 {
    pub fn new(input: &[u8]) -> Self {
        let mut buf = input;

        Self {
            data_format: buf.get_u8(),

            humidity: Some(buf.get_u8())
                .filter(|n| n <= &200)
                .map(|n| f32::from(n) * 0.5),

            temperature: {
                let int = buf.get_u8();
                Some(buf.get_u8()).filter(|n| n < &100).map(|n| {
                    if int & 0x80 == 0 {
                        f32::from(int) + f32::from(n) * 0.01
                    } else {
                        -(f32::from(int & 0x7f) + f32::from(n) * 0.01)
                    }
                })
            },

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

            voltage: Some(buf.get_u16())
                .filter(|n| (1600..=3646).contains(n))
                .map(|n| f32::from(n) * 0.001)
                .map(round),
        }
    }
}

fn round(n: f32) -> f32 {
    (n * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::Ruuvi3;

    const VALID_INPUT: &[u8; 14] = b"\x03\x29\x1A\x1E\xCE\x1E\xFC\x18\xF9\x42\x02\xCA\x0B\x53";

    const MAX_INPUT: &[u8; 14] = b"\x03\xC8\x7F\x63\xFF\xFE\x7F\xFF\x7F\xFF\x7F\xFF\x0E\x3E";

    const MIN_INPUT: &[u8; 14] = b"\x03\x00\xFF\x63\x00\x00\x80\x01\x80\x01\x80\x01\x06\x40";

    const INVALID_INPUT: &[u8; 14] = b"\x03\xFF\xFF\xFF\xFF\xFF\x80\x00\x80\x00\x80\x00\xFF\xFF";

    fn valid_output() -> Ruuvi3 {
        Ruuvi3 {
            data_format: 3,
            humidity: Some(20.5),
            temperature: Some(26.3),
            pressure: Some(102766),
            acceleration_x: Some(-1.0),
            acceleration_y: Some(-1.726),
            acceleration_z: Some(0.714),
            voltage: Some(2.899),
        }
    }

    fn max_output() -> Ruuvi3 {
        Ruuvi3 {
            data_format: 3,
            humidity: Some(100.0),
            temperature: Some(127.99),
            pressure: Some(115534),
            acceleration_x: Some(32.767),
            acceleration_y: Some(32.767),
            acceleration_z: Some(32.767),
            voltage: Some(3.646),
        }
    }

    fn min_output() -> Ruuvi3 {
        Ruuvi3 {
            data_format: 3,
            humidity: Some(0.0),
            temperature: Some(-127.99),
            pressure: Some(50000),
            acceleration_x: Some(-32.767),
            acceleration_y: Some(-32.767),
            acceleration_z: Some(-32.767),
            voltage: Some(1.6),
        }
    }

    fn invalid_output() -> Ruuvi3 {
        Ruuvi3 {
            data_format: 3,
            humidity: None,
            temperature: None,
            pressure: None,
            acceleration_x: None,
            acceleration_y: None,
            acceleration_z: None,
            voltage: None,
        }
    }

    #[test]
    fn valid() {
        let data = Ruuvi3::new(&VALID_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi3 = serde_json::from_str(&json).unwrap();
        assert_eq!(&valid_output(), &output);
    }

    #[test]
    fn maximum() {
        let data = Ruuvi3::new(&MAX_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi3 = serde_json::from_str(&json).unwrap();
        assert_eq!(&max_output(), &output);
    }

    #[test]
    fn minimum() {
        let data = Ruuvi3::new(&MIN_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi3 = serde_json::from_str(&json).unwrap();
        assert_eq!(&min_output(), &output);
    }

    #[test]
    fn invalid() {
        let data = Ruuvi3::new(&INVALID_INPUT[..]);
        let json = serde_json::to_string(&data).unwrap();
        let output: Ruuvi3 = serde_json::from_str(&json).unwrap();
        assert_eq!(&invalid_output(), &output);
    }
}
