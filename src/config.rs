use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

use crate::api::Result;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
    pub api_token: String,
    pub zone_id: String,
    pub subdomains: Vec<Subdomain>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Subdomain {
    pub name: String,
    pub a: bool,
    pub aaaa: bool,
    pub proxied: bool,
    pub ttl: u32,
}

impl Config {
    pub(crate) fn read_from(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub(crate) fn validate(self) -> Result<Self> {
        match self {
            c if c.api_token.is_empty() => Err("'api_token' cannot be empty'"),
            c if c.zone_id.is_empty() => Err("'zone_id' cannot be empty'"),
            c if c.subdomains.iter().any(|s| s.ttl < 300) => Err("'ttl' must be at least 300"),
            c => Ok(c),
        }
        .map_err(|s| Error::from(s).into())
    }

    pub(crate) fn needs_ipv6(&self) -> bool {
        self.subdomains.iter().any(|sd| sd.aaaa)
    }
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
