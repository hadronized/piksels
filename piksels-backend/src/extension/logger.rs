//! Logger extension.
//!
//! This extension allows to add logging capability to backends.

pub struct LoggerExt<F> {
  /// Filter used to filter logs.
  ///
  /// Only logs with a level less or equal to this level will be shown.
  pub level_filter: LogLevel,

  pub logger: F,
}

impl<F> LoggerExt<F> {
  pub fn new(level_filter: LogLevel, logger: F) -> Self {
    Self {
      level_filter,
      logger,
    }
  }
}

/// Logger implementation.
pub trait Logger {
  fn log(&self, log_entry: LogEntry);
}

/// Backends that can log.
///
/// Backends are supposed to call [`Logger::log`] to perform the actual logging on the provided logger.
pub trait BackendLogger {
  fn log(&self, log_entry: LogEntry);
}

/// Logger level.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LogLevel {
  Error,
  #[default]
  Warn,
  Info,
  Debug,
  Trace,
}

/// A log entry.
#[derive(Clone, Debug)]
pub struct LogEntry {
  pub level: LogLevel,
  pub file: &'static str,
  pub line: u32,
  pub column: u32,
  pub module: &'static str,
  pub msg: String,
}

#[macro_export]
macro_rules! log {
  ($backend:expr, $lvl:ident, $($msg:tt)*) => {
    #[cfg(feature = "ext-logger")]
    $backend.log($crate::extension::logger::LogEntry {
      level: $crate::extension::logger::LogLevel::$lvl,
      file: file!(),
      line: line!(),
      column: column!(),
      module: module_path!(),
      msg: format!($($msg)*),
    })
  }
}

#[macro_export]
macro_rules! trace {
  ($backend:expr, $($msg:tt)*) => {
    $crate::log!($backend, Trace, $($msg)*)
  }
}

#[macro_export]
macro_rules! debug {
  ($backend:expr, $($msg:tt)*) => {
    $crate::log!($backend, Debug, $($msg)*)
  }
}

#[macro_export]
macro_rules! info {
  ($backend:expr, $($msg:tt)*) => {
    $crate::log!($backend, Info, $($msg)*)
  }
}

#[macro_export]
macro_rules! warn {
  ($backend:expr, $($msg:tt)*) => {
    $crate::log!($backend, Warn, $($msg)*)
  }
}

#[macro_export]
macro_rules! error {
  ($backend:expr, $($msg:tt)*) => {
    $crate::log!($backend, Error, $($msg)*)
  }
}
