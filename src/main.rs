use std::str::FromStr;
use utils::error::Result;
use utils::logger;
use utils::types::LogLevel;

fn main() -> Result<()> {
    let cli = cli::Cli::from_env_and_args();

    let log_level_argument = cli.log_level.to_lowercase();
    let log_level = LogLevel::from_str(log_level_argument.as_str()).unwrap_or(LogLevel::Info);

    logger::initialize(&log_level);

    log::debug!("Slack API Key: {}", &cli.api_key);
    log::debug!("Output directory: {:?}", &cli.export_base_path);
    log::debug!("Delay: {}", &cli.request_delay);
    log::debug!("Log Level: {}", &cli.log_level);

    Ok(())
}
