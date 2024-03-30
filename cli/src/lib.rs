use clap::Parser;
use serde::Serialize;
use std::env;
use std::path::PathBuf;

fn get_default_output_directory() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("output");

    path
}

#[derive(Debug, Parser, Serialize)]
pub struct Cli {
    /// Slack API Key
    #[arg(short = 'k', long = "api-key", env = "SLACK_API_KEY")]
    pub api_key: String,

    /// Directory where channel messages are exported
    #[arg(
        short = 'o',
        long = "output",
        required = false,
        value_hint = clap::ValueHint::DirPath,
        default_value = get_default_output_directory().into_os_string()
    )]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub export_base_path: Option<std::path::PathBuf>,

    /// List of channels to export
    // #[arg(short = 'i', long = "include-channels", required = false, value_delimiter = ',')]
    // pub include_channels: Option<Vec<String>>,

    /// Default delay before resending a request when there is a throw limit
    #[arg(
        short = 'd',
        long = "request-delay",
        required = false,
        default_value = "1200"
    )]
    pub request_delay: i16,

    /// Default delay before resending a request when there is a throw limit
    #[arg(
        short = 'l',
        long = "log-level",
        required = false,
        default_value = "info"
    )]
    pub log_level: String,
}

impl Cli {
    pub fn from_env_and_args() -> Self {
        dotenvy::dotenv().ok();
        Self::parse()
    }
}
