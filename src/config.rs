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
