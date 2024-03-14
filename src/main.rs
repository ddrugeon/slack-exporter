use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use slack_exporter::cli;
use slack_exporter::config;
use slack_exporter::logger;

#[tokio::main]
async fn main() -> Result<(), figment::Error> {
    let conf: config::Config = Figment::new()
        .merge(Toml::file("Config.toml"))
        .merge(Env::prefixed("SLACK_"))
        .merge(Serialized::defaults(cli::Cli::from_env_and_args()))
        .extract()?;

    logger::initialize(&conf.log_level);

    log::debug!("Slack API Key: {}", conf.api_key);
    log::debug!("Log level: {}", conf.log_level);
    log::debug!("Export base path: {}", conf.export_base_path);
    log::debug!("Include channels: {:?}", conf.include_channels);
    log::debug!("Request delay: {}", conf.request_delay);
    Ok(())
}
