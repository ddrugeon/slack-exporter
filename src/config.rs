use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq, Debug, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            _ => Err(format!("Unknown log level: {s}")),
        }
    }
}

/// Global configuration for Slack-exporter
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// The Slack API Key
    pub api_key: String,

    /// Log Level: one of trace, debug, info, warning or error
    pub log_level: LogLevel,

    /// Directory where channel messages are exported
    pub export_base_path: String,

    /// List of channels to export
    pub include_channels: Vec<String>,

    /// Default delay before resending a request when there is a throw limit
    pub request_delay: i16,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            api_key: "default".into(),
            log_level: LogLevel::Info,
            export_base_path: "export".into(),
            include_channels: vec!["general".into()],
            request_delay: 1200,
        }
    }
}
