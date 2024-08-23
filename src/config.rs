use std::sync::OnceLock;

use anyhow::{anyhow, Error};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Password {
    pub secret_key: Option<String>,
    pub secret_jwt: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: Option<String>,
    pub max_conn: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct HTTP {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub password: Password,
    pub http: HTTP,
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Build the configuration from `local.toml` first
        let mut config = Config::builder()
            .add_source(File::with_name("local"))
            .build()?;

        // Extract the `debug` value
        let debug_mode: bool = config.get("debug").unwrap_or(false); // default to false if not found

        // Determine which configuration file to use based on the `debug` value
        let run_mode = if debug_mode {
            "development"
        } else {
            "production"
        };

        // Build the configuration again with the determined file
        let s = Config::builder()
            .add_source(File::with_name(run_mode))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }

    pub fn init() -> Result<(), Error> {
        let settings = Settings::new()?;
        SETTINGS
            .set(settings)
            .map_err(|_| anyhow!("Settings already initialized"))?;
        Ok(())
    }

    pub fn get() -> &'static Settings {
        SETTINGS.get().expect("Settings not initialized")
    }
}
