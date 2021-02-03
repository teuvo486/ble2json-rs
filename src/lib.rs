mod config;
mod dbus;
mod device;
mod server;

use config::Config;
use std::error::Error;
use std::path::Path;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

const CONF_PATH: &str = ".config/ble2json/config.json";

pub fn run() -> Result<()> {
    let home = std::env::var("HOME").expect("$HOME not set!");
    let path = Path::new(&home).join(CONF_PATH);
    let json = std::fs::read_to_string(path)?;
    let conf: Config = serde_json::from_str(&json)?;
    server::run(&conf)
}
