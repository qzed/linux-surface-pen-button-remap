use std::path::PathBuf;

use anyhow::Result;
use evdev_rs::enums::EV_KEY;
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_PATH: &str = "/etc/surface-pen-button/remap.conf";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub device: DeviceMatch,
    pub actions: Actions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceMatch {
    pub vendor: u16,
    pub product: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actions {
    pub single: Vec<EV_KEY>,
    pub double: Vec<EV_KEY>,
    pub hold: Vec<EV_KEY>,
}

pub fn load() -> Result<Config> {
    let path = std::env::var_os("SURFACE_PEN_BUTTON_CONFIG")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH));

    let config = std::fs::read_to_string(path)?;
    let config = toml::from_str(&config)?;

    Ok(config)
}
