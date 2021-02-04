use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub devices: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 8080,
            devices: {
                let mut hm = HashMap::new();
                hm.insert("example".to_string(), "CB:B8:33:4C:88:4F".to_string());
                hm
            },
        }
    }
}

impl Config {
    pub fn check_macs(&mut self) {
        let invalid: Vec<String> = self
            .devices
            .iter()
            .filter(|t| !is_mac(t.1))
            .map(|t| t.0.clone())
            .collect();

        for name in invalid.iter() {
            self.devices.remove(name);
            eprintln!("Invalid MAC address for device {}", name);
        }
    }
}

fn is_hex_byte(s: &&str) -> bool {
    s.len() == 2 && s.matches(|c: char| c.is_ascii_hexdigit()).count() == 2
}

fn is_mac(addr: &str) -> bool {
    addr.len() == 17 && addr.split(':').filter(is_hex_byte).count() == 6
}
