use std::time::Duration;

use super::config::Config;
use super::Result;
use dbus::arg::Variant;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::{BlockingSender, Connection};
use dbus::Message;
use std::collections::HashMap;

const TIMEOUT: u64 = 5;
const OBJ_BASE: &str = "/org/bluez/hci0/dev_";

pub type MfgData = HashMap<u16, Variant<Vec<u8>>>;

pub struct Conn {
    conn: Connection,
    timeout: Duration,
    obj_paths: HashMap<String, String>,
}

impl Conn {
    pub fn new(conf: &Config) -> Result<Self> {
        let mut s = Self {
            conn: Connection::new_system()?,
            timeout: Duration::from_secs(TIMEOUT),
            obj_paths: HashMap::new(),
        };

        s.add_obj_paths(conf);
        s.start_discovery()?;
        Ok(s)
    }

    fn start_discovery(&self) -> Result<()> {
        let message = Message::new_method_call(
            "org.bluez",
            "/org/bluez/hci0",
            "org.bluez.Adapter1",
            "StartDiscovery",
        )?;

        self.conn.send_with_reply_and_block(message, self.timeout)?;
        Ok(())
    }

    fn add_obj_paths(&mut self, conf: &Config) {
        for addr in conf.devices.values() {
            let mut path = OBJ_BASE.to_string();
            path.push_str(&addr.replace(":", "_"));
            self.obj_paths.insert(addr.to_string(), path);
        }
    }

    pub fn get_rssi(&self, addr: &str) -> Option<i16> {
        let obj_path = self.obj_paths.get(addr).unwrap();
        let proxy = self.conn.with_proxy("org.bluez", obj_path, self.timeout);
        proxy.get("org.bluez.Device1", "RSSI").ok()
    }

    pub fn get_mfg_data(&self, addr: &str) -> Option<MfgData> {
        let obj_path = self.obj_paths.get(addr).unwrap();
        let proxy = self.conn.with_proxy("org.bluez", obj_path, self.timeout);
        proxy
            .get("org.bluez.Device1", "ManufacturerData")
            .ok()
            .filter(|d: &MfgData| d.len() == 1)
    }
}
