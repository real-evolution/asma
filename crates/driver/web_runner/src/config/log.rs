use kernel_services::{config::ConfigService, get_config};

use anyhow::Result;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;
use tracing::metadata::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::EnvFilter;
use validator::Validate;

const CONFIG_SECTION: &str = "log";
const ENV_LOG_KEY: &str = "ASMA_LOG";

into_fn!(default_show_file: const bool => false);
into_fn!(default_show_target: const bool => true);
into_fn!(default_show_line: const bool => false);
into_fn!(default_show_thread_id: const bool => false);
into_fn!(default_show_thread_names: const bool => false);
into_fn!(default_show_level: const bool => true);
into_fn!(default_use_ansi: const bool => true);
into_fn!(default_formatter: const LogFormatter => LogFormatter::Full);

const fn default_level() -> LogLevel {
    if cfg!(debug_assertions) {
        LogLevel::Trace
    } else {
        LogLevel::Trace
    }
}

#[derive(Debug, Deserialize, Validate)]
struct LogConfig {
    #[serde(default = "default_show_file")]
    show_file: bool,
    #[serde(default = "default_show_target")]
    show_target: bool,
    #[serde(default = "default_show_line")]
    show_line: bool,
    #[serde(default = "default_show_thread_id")]
    show_thread_id: bool,
    #[serde(default = "default_show_thread_names")]
    show_thread_names: bool,
    #[serde(default = "default_show_level")]
    show_level: bool,
    #[serde(default = "default_use_ansi")]
    use_ansi: bool,
    #[serde(default = "default_formatter")]
    formatter: LogFormatter,
    #[serde(default = "default_level")]
    level: LogLevel,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            show_file: default_show_file(),
            show_target: default_show_target(),
            show_line: default_show_line(),
            show_thread_id: default_show_thread_id(),
            show_thread_names: default_show_thread_names(),
            show_level: default_show_level(),
            use_ansi: default_use_ansi(),
            formatter: default_formatter(),
            level: default_level(),
        }
    }
}

#[derive(Debug, Deserialize_enum_str)]
#[serde(rename_all = "snake_case")]
enum LogFormatter {
    Full,
    Compact,
    Pretty,
    Json,
}

#[derive(Debug, Deserialize_enum_str)]
#[serde(rename_all = "snake_case")]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[serde(rename = "error")]
    Critical,
    Off,
}

impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Critical => LevelFilter::ERROR,
            LogLevel::Off => LevelFilter::OFF,
        }
    }
}

pub fn configure_logger_with<'a, C: ConfigService + ?Sized>(
    svc: &'a C,
) -> Result<()> {
    let mut result = Ok(());

    let conf =
        get_config!(svc, CONFIG_SECTION => LogConfig).unwrap_or_else(|e| {
            result = Err(e);
            LogConfig::default()
        });

    let fmt = tracing_subscriber::fmt()
        .with_file(conf.show_file)
        .with_target(conf.show_target)
        .with_line_number(conf.show_line)
        .with_thread_ids(conf.show_thread_id)
        .with_thread_names(conf.show_thread_names)
        .with_ansi(conf.use_ansi)
        .with_level(conf.show_level)
        .with_env_filter(EnvFilter::from_env(ENV_LOG_KEY))
        .with_max_level(conf.level);

    match conf.formatter {
        LogFormatter::Full => set_global_default(fmt.finish()),
        LogFormatter::Compact => set_global_default(fmt.compact().finish()),
        LogFormatter::Pretty => set_global_default(fmt.pretty().finish()),
        LogFormatter::Json => set_global_default(fmt.json().finish()),
    }?;

    if let Err(err) = result {
        warn!("unable use user-defined logger config, using defaults: {err}");
    }

    Ok(())
}
