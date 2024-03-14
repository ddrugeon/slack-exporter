use crate::types;
use flexi_logger::{DeferredNow, Duplicate};

/// Initializes our custom logger.
/// Logs to stdout and stderr.
pub fn initialize(level: &types::LogLevel) {
    flexi_logger::Logger::try_with_str(level.to_string())
        .unwrap()
        .log_to_stdout()
        .format(
            |w: &mut dyn std::io::Write, now: &mut DeferredNow, record: &log::Record| {
                write!(
                    w,
                    "{} [{}] {}",
                    now.format("%Y-%m-%d %H:%M:%S%.6f"),
                    record.level(),
                    &record.args()
                )
            },
        )
        .duplicate_to_stderr(Duplicate::Error)
        .append()
        .start()
        .unwrap();
}
