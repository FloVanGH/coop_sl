// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

const SETTINGS_EXT: &str = ".toml";

#[derive(Clone, Debug)]
pub struct SettingsService {
    location: String,
}

impl SettingsService {
    pub fn new() -> io::Result<Self> {
        let config = dirs::config_dir()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Cannot read config dir.",
            ))?
            .join("coop_local");

        if !config.exists() {
            fs::create_dir_all(config.clone())?;
        }

        Ok(Self {
            location: config
                .to_str()
                .ok_or(io::Error::new(
                    io::ErrorKind::Other,
                    "Cannot read config dir.",
                ))?
                .into(),
        })
    }

    pub fn save<V>(&self, key: impl Into<String>, value: &V) -> io::Result<()>
    where
        V: serde::Serialize,
    {
        let mut settings_path = Path::new(self.location.as_str()).to_path_buf();
        let mut key = key.into();
        key.push_str(SETTINGS_EXT);
        settings_path = settings_path.join(key);

        let value_toml = toml::to_string(&value).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Save settings error: {}", e.to_string()),
            )
        })?;

        let mut settings_file = fs::File::create(settings_path)?;
        settings_file.write_all(value_toml.as_bytes())
    }

    pub fn load<V>(&self, key: impl Into<String>) -> io::Result<V>
    where
        V: serde::de::DeserializeOwned,
    {
        let mut settings_path = Path::new(self.location.as_str()).to_path_buf();
        let mut key = key.into();
        key.push_str(SETTINGS_EXT);
        settings_path = settings_path.join(key);

        let mut content = String::default();

        let mut file = fs::File::open(settings_path)?;
        file.read_to_string(&mut content)?;

        toml::from_str(content.as_str()).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Load settings error: {}", e.to_string()),
            )
        })
    }
}
