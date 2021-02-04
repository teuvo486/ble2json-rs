mod ruuvi3;
mod ruuvi5;

use super::dbus::{Conn, MfgData};
use super::Result;
use ruuvi3::Ruuvi3;
use ruuvi5::Ruuvi5;
use serde::Serialize;
use std::io::Cursor;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
enum SensorData {
    Ruuvi3(Ruuvi3),
    Ruuvi5(Ruuvi5),
}

impl SensorData {
    fn new(dict: MfgData) -> Option<Self> {
        let (id, v) = dict.iter().next().unwrap();
        match (id, v.0.get(0), v.0.len()) {
            (0x0499, Some(3), 14) => Some(Self::Ruuvi3(Ruuvi3::new(&v.0))),
            (0x0499, Some(5), 24) => Some(Self::Ruuvi5(Ruuvi5::new(&v.0))),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    name: String,
    address: String,
    rssi: Option<i16>,
    sensor_data: Option<SensorData>,
}

impl Data {
    pub fn new(name: &str, addr: &str, conn: &Conn) -> Self {
        Self {
            name: name.to_string(),
            address: addr.to_string(),
            rssi: conn.get_rssi(&addr),
            sensor_data: conn.get_mfg_data(&addr).and_then(SensorData::new),
        }
    }

    pub fn to_json(&self, output: &mut Vec<u8>) -> Result<()> {
        let mut writer = Cursor::new(output);
        serde_json::to_writer(&mut writer, self)?;
        Ok(())
    }
}
