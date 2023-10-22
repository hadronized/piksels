//! Cache helpers for backends.
//!
//! Even though it is not mandatory for backends to cache their commands, caching often allows to prevent costly
//! change on the graphics device. This module exports the [`Cached`] helper function, along with a simple cache for
//! querying backend information. The rest is implementation details.

use crate::BackendInfo;

/// Cache for query information.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct QueryCache {
  author: Option<String>,
  name: Option<String>,
  version: Option<String>,
  shading_lang_version: Option<String>,
  info: Option<BackendInfo>,
}

impl QueryCache {
  pub fn author(&mut self) -> &mut Option<String> {
    &mut self.author
  }

  pub fn name(&mut self) -> &mut Option<String> {
    &mut self.name
  }

  pub fn version(&mut self) -> &mut Option<String> {
    &mut self.version
  }

  pub fn shading_lang_version(&mut self) -> &mut Option<String> {
    &mut self.shading_lang_version
  }

  pub fn info(&mut self) -> &mut Option<BackendInfo> {
    &mut self.info
  }
}

/// Cached value.
///
/// A cached value is used to prevent issuing costy GPU commands if we know the target value is
/// already set to what the command tries to set. For instance, if you ask to use texture ID
/// `34` once, that value will be set on the GPU and cached on our side. Later, if no other texture
/// setting has occurred, if you ask to use the texture ID `34` again, because the value is cached,
/// we know the GPU is already using it, so we don’t have to perform anything GPU-wise.
///
/// This optimization has limits and sometimes, because of side-effects, it is not possible to cache
/// something correctly.
#[derive(Debug)]
pub struct Cached<T>(Option<T>);

impl<T> Default for Cached<T> {
  fn default() -> Self {
    Cached(None)
  }
}

impl<T> Cached<T>
where
  T: PartialEq + Clone,
{
  /// Explicitly invalidate a value.
  pub fn invalidate(&mut self) {
    self.0 = None;
  }

  /// Explicitly set a value.
  pub fn set(&mut self, value: T) -> Option<T> {
    self.0.replace(value)
  }

  /// Set the value if invalid, then call the function.
  ///
  /// If the value was still valid, returns `true`.
  ///
  /// See more: [`Cached::is_invalid`].
  pub fn set_if_invalid<E>(
    &mut self,
    value: &T,
    f: impl FnOnce() -> Result<(), E>,
  ) -> Result<bool, E> {
    match self.0 {
      Some(ref x) if x == value => Ok(false),

      _ => {
        self.0 = Some(value.clone());
        f().map(|_| true)
      }
    }
  }

  /// Check whether a value is cached, whatever it is.
  pub fn exists(&self) -> bool {
    self.0.is_some()
  }

  /// Check whether the cached value is invalid regarding a value.
  ///
  /// A value is invalid if it was never set, or if it’s different from the parameter one.
  pub fn is_invalid(&self, new_val: &T) -> bool {
    match &self.0 {
      Some(ref t) => t != new_val,
      _ => true,
    }
  }
}
