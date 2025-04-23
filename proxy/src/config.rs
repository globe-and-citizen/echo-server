use std::fs;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub upstream: UpstreamConfig,
    pub log: LogConfig,
    pub server: ServerConfig,
    pub crypto: CryptoConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read configuration file");
        toml::from_str(&content).expect("Failed to parse configuration file")
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct UpstreamConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub(super) struct LogConfig {
    pub path: String,
    pub level: String,
}

impl LogConfig {
    pub fn to_level_filter(&self) -> log::LevelFilter {
        match self.level.as_str() {
            "INFO" => log::LevelFilter::Info,
            "DEBUG" => log::LevelFilter::Debug,
            "WARNING" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,
            "TRACE" => log::LevelFilter::Trace,
            "OFF" => log::LevelFilter::Off,
            _ => log::max_level()
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct ServerConfig {
    pub address: String
}

#[derive(Debug, Deserialize)]
pub(super) struct CryptoConfig {
    pub secret: String,
}

