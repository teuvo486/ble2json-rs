use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test() {
        let data = Config::default();
        let json = serde_json::to_string_pretty(&data).unwrap();
        let output: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(&data, &output);
    }
}
