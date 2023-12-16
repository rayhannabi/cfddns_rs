use std::collections::HashMap;

use crate::error::CloudflareError;

use super::*;

pub async fn public_ipv4() -> Result<String> {
    public_ip(CLOUDFLARE_IPV4, CLOUDFLARE_IPV4_ALT, false).await
}

pub async fn public_ipv6() -> Result<String> {
    public_ip(CLOUDFLARE_IPV6, CLOUDFLARE_IPV6_ALT, true).await
}

async fn public_ip(cf_host: &str, alt_host: &str, is_ipv6: bool) -> Result<String> {
    let url = format!("{}{}", cf_host, CLOUDFLARE_TRACE_PATH);
    let response = cf_trace(&url).await;
    let ip_type: &str = if is_ipv6 { "IPv6" } else { "IPv4" };
    match response {
        Ok(ip) => {
            log::info!("🌐 Public {}: {}", ip_type, ip);
            Ok(ip)
        }
        Err(e) => {
            cf_host_error(e, is_ipv6);
            log::info!("Trying alternative url...");
            let alt_url = format!("{}{}", alt_host, CLOUDFLARE_TRACE_PATH);
            cf_trace(&alt_url)
                .await
                .map_err(|e| cf_host_error(e, is_ipv6))
        }
    }
}

async fn cf_trace(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?.text().await?;
    let ip = parse_ip_from(response)?;
    Ok(ip)
}

fn parse_ip_from(trace: String) -> Result<String> {
    let iter = trace.trim().split("\n").filter_map(|s| s.split_once("="));
    let map: HashMap<_, _> = HashMap::from_iter(iter);

    match map.get("ip") {
        Some(value) => Ok(value.to_string()),
        None => Err(CloudflareError::IpParseError),
    }
}

pub fn cf_host_error(error: CloudflareError, is_ipv6: bool) -> CloudflareError {
    if let CloudflareError::HostError(ref e) = error {
        match e {
            e if e.is_connect() || e.is_request() => {
                let ip_type = if is_ipv6 { "IPv6" } else { "IPv4" };
                log::error!(
                    "Cloudflare ({}) is not reachable via {}",
                    e.url().unwrap().host_str().unwrap(),
                    ip_type
                );
            }
            e => log::error!("{}", e),
        }
    } else {
        log::error!("{}", error);
    }
    error
}
