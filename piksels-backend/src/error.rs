use std::sync::PoisonError;

use thiserror::Error;

/// Backend common errors.
///
/// Backend errors are specific for each technology they wrap. However, they are some overlapping kind of errors that
/// backend can hit. Backend error types must then implement [`From`] this type.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  #[error("no more units available on device")]
  NoMoreUnits,

  #[error("thread is poisoned")]
  PoisonedThread,

  #[error("extension check failed: {reason}")]
  ExtensionCheck { reason: String },
}

impl<T> From<PoisonError<T>> for Error {
  fn from(_: PoisonError<T>) -> Self {
    Error::PoisonedThread
  }
}
