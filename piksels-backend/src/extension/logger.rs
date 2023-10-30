//! Logger extension.
//!
//! This extension allows to add logging capability to backends.

/// Logger state.
pub struct Logger {
  /// Filter used to filter logs.
  ///
  /// Only logs with a level less or equal to this level will be shown.
  level_filter: LogLevel,

  logger: Box<dyn Fn(LogEntry)>,
}

impl Logger {
  pub fn new(level_filter: LogLevel, logger: impl 'static + Fn(LogEntry)) -> Self {
    Self {
      level_filter,
      logger: Box::new(logger),
    }
  }

  pub fn set_level_filter(&mut self, level_filter: LogLevel) {
    self.level_filter = level_filter;
  }

  pub fn level_filter(&self) -> LogLevel {
    self.level_filter
  }

  pub fn logger(&self) -> &impl Fn(LogEntry) {
    &self.logger
  }
}

/// Backends that can log.
pub trait ExtLogger {
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
