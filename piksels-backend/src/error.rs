use thiserror::Error;

/// Backend common errors.
///
/// Backend errors are specific for each technology they wrap. However, they are some overlapping kind of errors that
/// backend can hit. Backend error types must then implement [`From`] this type.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  #[error("no more units available on device")]
  NoMoreUnits,
}
