use std::{env, path::PathBuf};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};
pub fn init_logging(logging_dir: PathBuf) -> anyhow::Result<()> {
    let logging_dir = match env::var("LOGGING_DIR") {
        Ok(ok) => PathBuf::from(ok),
        Err(_) => logging_dir,
    };

    if !logging_dir.exists() {
        std::fs::create_dir_all(&logging_dir)?;
    }
    let warn_log = {
        let warning_logs = tracing_appender::rolling::Builder::new()
            .rotation(tracing_appender::rolling::Rotation::HOURLY)
            .filename_prefix("warnings")
            .filename_suffix(".log")
            .build(logging_dir.clone())?;
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_file(true)
            .with_level(true)
            .with_writer(warning_logs)
            .with_filter(LevelFilter::WARN)
    };

    // Create a tracing layer with the configured tracer
    let log_level = LevelFilter::TRACE;

    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(filter::Targets::new().with_target("nitro_launcher", log_level));

    tracing_subscriber::registry()
        .with(warn_log)
        .with(stdout_log)
        .try_init()?;

    Ok(())
}
