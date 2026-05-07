use clap::ValueEnum;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

static LOGGER: OnceLock<Option<LogState>> = OnceLock::new();

struct LogState {
    level: LogLevel,
    path: PathBuf,
    _stderr_guard: WorkerGuard,
    _file_guard: WorkerGuard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    fn from_env(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "error" => Self::Error,
            "warn" | "warning" => Self::Warn,
            "debug" => Self::Debug,
            _ => Self::Info,
        }
    }
}

impl LogLevel {
    fn as_tracing_level(self) -> Level {
        match self {
            Self::Error => Level::ERROR,
            Self::Warn => Level::WARN,
            Self::Info => Level::INFO,
            Self::Debug => Level::DEBUG,
        }
    }

    fn as_filter(self) -> LevelFilter {
        match self {
            Self::Error => LevelFilter::ERROR,
            Self::Warn => LevelFilter::WARN,
            Self::Info => LevelFilter::INFO,
            Self::Debug => LevelFilter::DEBUG,
        }
    }
}

fn effective_level(debug: bool, level: Option<LogLevel>) -> Option<LogLevel> {
    if let Some(level) = level {
        return Some(level);
    }
    if debug {
        return Some(LogLevel::Debug);
    }
    std::env::var("ZENTAO_LOG")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(|v| LogLevel::from_env(&v))
}

fn default_log_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(dir) = dirs::data_local_dir() {
            return dir.join("zentao-cli").join("logs");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            return home.join("Library").join("Logs").join("zentao-cli");
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(state_home) = std::env::var("XDG_STATE_HOME") {
            if !state_home.trim().is_empty() {
                return PathBuf::from(state_home).join("zentao-cli").join("logs");
            }
        }
        if let Some(home) = dirs::home_dir() {
            return home
                .join(".local")
                .join("state")
                .join("zentao-cli")
                .join("logs");
        }
    }

    PathBuf::from(".zentao-cli").join("logs")
}

pub fn default_log_path() -> PathBuf {
    default_log_dir().join("zentao-cli.log")
}

pub fn default_log_file_pattern() -> &'static str {
    "zentao-cli.log.YYYY-MM-DD"
}

pub fn init(debug: bool, level: Option<LogLevel>) {
    let level = effective_level(debug, level);
    let logger = level.and_then(|level| {
        let dir = default_log_dir();
        if create_dir_all(&dir).is_err() {
            return None;
        }

        let (stderr_writer, stderr_guard) = tracing_appender::non_blocking(std::io::stderr());
        let file_appender = rolling::daily(&dir, "zentao-cli.log");
        let (file_writer, file_guard) = tracing_appender::non_blocking(file_appender);
        let level_filter = level.as_filter();

        let stderr_layer = fmt::layer()
            .with_writer(stderr_writer)
            .with_target(true)
            .with_ansi(false)
            .with_filter(level_filter);

        let file_layer = fmt::layer()
            .with_writer(file_writer)
            .with_target(true)
            .with_ansi(false)
            .with_filter(level_filter);

        tracing_subscriber::registry()
            .with(stderr_layer)
            .with(file_layer)
            .try_init()
            .ok()?;

        Some(LogState {
            level,
            path: dir.join(default_log_file_pattern()),
            _stderr_guard: stderr_guard,
            _file_guard: file_guard,
        })
    });

    let _ = LOGGER.set(logger);
}

pub fn current_log_path() -> Option<PathBuf> {
    LOGGER
        .get()
        .and_then(|state| state.as_ref().map(|state| state.path.clone()))
}

pub fn is_enabled(level: LogLevel) -> bool {
    match LOGGER.get() {
        Some(Some(state)) => level <= state.level,
        _ => false,
    }
}

pub fn log(level: LogLevel, scope: &str, message: impl AsRef<str>) {
    if !is_enabled(level) {
        return;
    }

    let rendered = format!("{} {}", scope, message.as_ref());

    match level.as_tracing_level() {
        Level::ERROR => tracing::error!("{}", rendered),
        Level::WARN => tracing::warn!("{}", rendered),
        Level::INFO => tracing::info!("{}", rendered),
        Level::DEBUG => tracing::debug!("{}", rendered),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{effective_level, LogLevel};
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn explicit_level_overrides_debug_flag() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            std::env::remove_var("ZENTAO_LOG");
        }

        assert_eq!(
            effective_level(true, Some(LogLevel::Warn)),
            Some(LogLevel::Warn)
        );
        assert_eq!(
            effective_level(true, Some(LogLevel::Error)),
            Some(LogLevel::Error)
        );
    }

    #[test]
    fn debug_flag_enables_debug_without_explicit_level() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            std::env::remove_var("ZENTAO_LOG");
        }

        assert_eq!(effective_level(true, None), Some(LogLevel::Debug));
    }

    #[test]
    fn env_level_is_used_when_flags_are_absent() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            std::env::set_var("ZENTAO_LOG", "warn");
        }

        assert_eq!(effective_level(false, None), Some(LogLevel::Warn));

        unsafe {
            std::env::remove_var("ZENTAO_LOG");
        }
    }

    #[test]
    fn test_log_level_from_env() {
        assert_eq!(LogLevel::from_env("error"), LogLevel::Error);
        assert_eq!(LogLevel::from_env("warn"), LogLevel::Warn);
        assert_eq!(LogLevel::from_env("warning"), LogLevel::Warn);
        assert_eq!(LogLevel::from_env("info"), LogLevel::Info);
        assert_eq!(LogLevel::from_env("debug"), LogLevel::Debug);
        assert_eq!(LogLevel::from_env("unknown"), LogLevel::Info);
    }

    #[test]
    fn test_log_level_as_tracing_level() {
        assert_eq!(LogLevel::Error.as_tracing_level(), tracing::Level::ERROR);
        assert_eq!(LogLevel::Warn.as_tracing_level(), tracing::Level::WARN);
        assert_eq!(LogLevel::Info.as_tracing_level(), tracing::Level::INFO);
        assert_eq!(LogLevel::Debug.as_tracing_level(), tracing::Level::DEBUG);
    }

    #[test]
    fn test_log_level_as_filter() {
        use tracing_subscriber::filter::LevelFilter;
        assert_eq!(LogLevel::Error.as_filter(), LevelFilter::ERROR);
        assert_eq!(LogLevel::Warn.as_filter(), LevelFilter::WARN);
        assert_eq!(LogLevel::Info.as_filter(), LevelFilter::INFO);
        assert_eq!(LogLevel::Debug.as_filter(), LevelFilter::DEBUG);
    }

    #[test]
    fn test_effective_level_none_both() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            std::env::remove_var("ZENTAO_LOG");
        }
        assert_eq!(effective_level(false, None), None);
    }

    #[test]
    fn test_log_level_debug() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            std::env::remove_var("ZENTAO_LOG");
        }
        assert_eq!(effective_level(true, None), Some(LogLevel::Debug));
    }

    #[test]
    fn test_log_level_explicit() {
        let _guard = env_lock().lock().unwrap();
        unsafe {
            std::env::remove_var("ZENTAO_LOG");
        }
        assert_eq!(
            effective_level(false, Some(LogLevel::Error)),
            Some(LogLevel::Error)
        );
    }
}
