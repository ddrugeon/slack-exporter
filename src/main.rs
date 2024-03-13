use std::error::Error;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};

// main.rs
use crate::cli::Cli;
use crate::config::Config;

pub mod cli;
pub mod config;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conf: Config = Figment::new()
        .merge(Toml::file("Config.toml"))
        .merge(Env::prefixed("SLACK_"))
        .merge(Serialized::defaults(Cli::from_env_and_args()))
        .extract()?;

    logger::initialize(&conf.log_level);

    log::debug!("Slack API Key: {}", conf.api_key);
    log::debug!("Log level: {}", conf.log_level);
    log::debug!("Export base path: {}", conf.export_base_path);
    log::debug!("Include channels: {:?}", conf.include_channels);
    log::debug!("Request delay: {}", conf.request_delay);
    Ok(())
}