use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

const CARGO_CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SteamVRConfig {
    pub register: bool,
}

impl Default for SteamVRConfig {
    fn default() -> Self {
        Self { register: true }
    }
}

impl SteamVRConfig {
    pub fn get_path() -> Result<PathBuf> {
        let mut path = std::env::current_exe()?;
        path.set_file_name(CARGO_CRATE_NAME);
        path.set_extension("toml");

        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut text = String::new();
        file.read_to_string(&mut text)?;
        file.rewind()?;

        match toml::from_str(&text) {
            Ok(config) => Ok(config),
            Err(_) => {
                let config = SteamVRConfig::default();
                let text = toml::to_string_pretty(&config)?;
                file.write_all(text.as_bytes())?;

                Ok(config)
            }
        }
    }
}
