use clap::Parser;

use crate::{api::trace, config::Config};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(
        short = 'c',
        help = "Path to JSON configuration file",
        value_parser = parser::config_file
    )]
    config: Config,

    #[arg(
        short = 't',
        help = "Refresh interval in seconds (minimum 5)",
        default_value_t = 300,
        value_parser = parser::refresh_interval
    )]
    refresh_interval: u32,
}

mod parser {
    use std::path::PathBuf;

    use crate::config::Config;

    pub fn config_file(path: &str) -> Result<Config, String> {
        let path = PathBuf::from(path);
        match path {
            p if !p.exists() => Err("The specified file does not exist".to_string()),
            p if !p.extension().is_some_and(|ext| ext == "json") => {
                Err("The configuration must be a JSON file".to_string())
            }
            _ => Config::read_from(&path)
                .and_then(|c| c.validate())
                .map_err(|e| e.to_string()),
        }
    }

    pub fn refresh_interval(value: &str) -> Result<u32, String> {
        let parsed = value.trim().parse::<u32>().map_err(|e| e.to_string())?;
        match parsed {
            x if x >= 5 => Ok(x),
            _ => Err("Refresh interval cannot be smaller than 5 seconds".to_string()),
        }
    }
}

#[tokio::main]
pub(crate) async fn run() {
    let args = Args::parse();
    println!("{:#?} {}", args.config, args.refresh_interval);

    let needs_ipv6 = args.config.needs_ipv6();
    let (ipv4, ipv6) = public_ip_addresses(needs_ipv6).await;
}

async fn public_ip_addresses(needs_ipv6: bool) -> (String, Option<String>) {
    let ipv4 = match trace::public_ipv4().await {
        Ok(addr) => addr,
        Err(_) => std::process::exit(1),
    };
    let ipv6 = match needs_ipv6 {
        true => trace::public_ipv6().await.ok(),
        false => None,
    };
    (ipv4, ipv6)
}
