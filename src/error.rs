use std::fmt::Display;

use crate::config;

#[derive(Debug)]
pub enum CloudflareError {
    HostError(reqwest::Error),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ConfigError(config::Error),
    IpParseError,
}

impl Display for CloudflareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudflareError::HostError(e) => write!(f, "{}", e),
            CloudflareError::IoError(e) => write!(f, "{}", e),
            CloudflareError::JsonError(e) => write!(f, "{}", e),
            CloudflareError::ConfigError(e) => write!(f, "{}", e.msg),
            CloudflareError::IpParseError => write!(f, "Could not parse the IP address"),
        }
    }
}

impl std::error::Error for CloudflareError {}

impl From<reqwest::Error> for CloudflareError {
    fn from(value: reqwest::Error) -> Self {
        CloudflareError::HostError(value)
    }
}

impl From<std::io::Error> for CloudflareError {
    fn from(value: std::io::Error) -> Self {
        CloudflareError::IoError(value)
    }
}

impl From<serde_json::Error> for CloudflareError {
    fn from(value: serde_json::Error) -> Self {
        CloudflareError::JsonError(value)
    }
}

impl From<config::Error> for CloudflareError {
    fn from(value: config::Error) -> Self {
        CloudflareError::ConfigError(value)
    }
}
