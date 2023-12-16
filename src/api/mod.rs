use crate::error::CloudflareError;

pub mod trace;

pub const CLOUDFLARE_IPV4: &str = "https://1.1.1.1";
pub const CLOUDFLARE_IPV4_ALT: &str = "https://1.0.0.1";
pub const CLOUDFLARE_IPV6: &str = "https://[2606:4700:4700::1111]";
pub const CLOUDFLARE_IPV6_ALT: &str = "https://[2606:4700:4700::1001]";
const CLOUDFLARE_TRACE_PATH: &str = "/cdn-cgi/trace";

pub type Result<T> = std::result::Result<T, CloudflareError>;
