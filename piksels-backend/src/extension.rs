use self::logger::LoggerExt;

#[cfg(feature = "ext-logger")]
pub mod logger;

pub struct ExtensionsBuilder<ExtLogger> {
  pub logger: ExtLogger,
}

impl Default for ExtensionsBuilder<()> {
  fn default() -> Self {
    Self { logger: () }
  }
}

impl ExtensionsBuilder<()> {
  pub fn logger<F>(self, logger: LoggerExt<F>) -> ExtensionsBuilder<LoggerExt<F>> {
    ExtensionsBuilder { logger }
  }
}
