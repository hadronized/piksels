use thiserror::Error;

use crate::Backend;

/// Backend common errors.
///
/// Backend errors are specific for each technology they wrap. However, they are some overlapping kind of errors that
/// backend can hit. Backend error types must then implement [`From`] this type.
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error<B>
where
  B: ?Sized + Backend,
{
  #[error("no more units available on device (max: {max_units})")]
  NoMoreUnits { max_units: B::Unit },
}
