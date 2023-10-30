use crate::Backend;

#[cfg(feature = "ext-logger")]
pub mod logger;

pub trait Extension<E>: Backend {
  /// Extension name, which should be unique to the extension.
  const NAME: &'static str;

  /// Initialize the extension.
  ///
  /// Some extensions might require to perform some runtime checks to know whether the extension is available. If those
  /// fail, the extension should fail with `Err(Error::ExtensionCheck)`.
  fn init_ext(&mut self, env: E) -> Result<(), Self::Err>;
}

/// Initialize many extensions.
///
/// Extensions are initialized in the order they are passed to this macro.
#[macro_export]
macro_rules! extensions {
  ($backend:ident, [$($ext:expr),* $(,)?]) => {{
    $(
      $backend.init_ext($ext)?;
    )*

    Ok(())
  }}
}
