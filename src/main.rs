mod api;
mod cli;
mod config;
mod error;

use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    cli::run();
}
