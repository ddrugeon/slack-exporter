use utils::app_config::AppConfig;
use utils::error::Result;
use utils::logger;

fn main() -> Result<()> {
    let config_contents = include_str!("resources/default_config.toml");
    AppConfig::init(Some(config_contents))?;

    let config = AppConfig::fetch()?;
    logger::initialize(&config.log_level);

    log::debug!("Log level: {}", config.log_level);
    Ok(())
}
