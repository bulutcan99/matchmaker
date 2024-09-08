use std::sync::OnceLock;

use anyhow::{anyhow, Error};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub enable: bool,
    pub pretty_backtrace: bool,
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct Middleware {
    pub enable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Middlewares {
    pub etag: Middleware,
    pub limit_payload: LimitPayload,
    pub secure_headers: SecureHeaders,
    pub remote_ip: Middleware,
    pub logger: Middleware,
    pub catch_panic: Middleware,
    pub timeout_request: TimeoutRequest,
    pub cors: Middleware,
}

#[derive(Debug, Deserialize)]
pub struct LimitPayload {
    pub enable: bool,
    pub body_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct SecureHeaders {
    pub preset: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeoutRequest {
    pub enable: bool,
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
    pub middlewares: Middlewares,
}

#[derive(Debug, Deserialize)]
pub struct Workers {
    pub mode: String,
}

#[derive(Debug, Deserialize)]
pub struct Mailer {
    pub enable: bool,
    pub host: String,
    pub port: u16,
    pub secure: bool,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub uri: String,
    pub enable_logging: bool,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub min_connections: u32,
    pub max_connections: u32,
    pub auto_migrate: bool,
    pub dangerously_truncate: bool,
    pub dangerously_recreate: bool,
}

#[derive(Debug, Deserialize)]
pub struct Queue {
    pub uri: String,
    pub dangerously_flush: bool,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub expiration: u64,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub jwt: Jwt,
    pub password: Password,
}

#[derive(Debug, Deserialize)]
pub struct Password {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub logger: Logger,
    pub server: Server,
    pub workers: Workers,
    pub mailer: Mailer,
    pub database: Database,
    pub queue: Queue,
    pub auth: Auth,
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            .add_source(File::with_name("config/local"))
            .build()?;

        let debug_mode: bool = config.get("debug").unwrap_or(false);
        let run_mode = if debug_mode {
            "config/development"
        } else {
            "config/production"
        };

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
