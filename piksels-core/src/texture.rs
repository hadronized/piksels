use std::sync::{Mutex, Weak};

use piksels_backend::{
  texture::{Rect, Size},
  Backend,
};

use crate::cache::ScarceCache;

#[derive(Debug, Eq, PartialEq)]
pub struct TextureUnit<B>
where
  B: Backend,
{
  pub(crate) raw: B::TextureUnit,
}

#[derive(Debug)]
pub struct Texture<B>
where
  B: Backend,
{
  pub(crate) raw: B::Texture,
  cache: Weak<Mutex<ScarceCache<B>>>,
}

impl<B> Drop for Texture<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    if let Some(Ok(mut cache)) = self.cache.upgrade().map(|c| c.lock()) {
      cache.untrack_texture(&self.raw);
    }
  }
}

impl<B> Texture<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::Texture, cache: Weak<Mutex<ScarceCache<B>>>) -> Self {
    Self { raw, cache }
  }

  pub fn resize(&self, size: Size) -> Result<(), B::Err> {
    B::resize_texture(&self.raw, size)
  }

  pub fn set(
    &self,
    rect: Rect,
    mipmaps: bool,
    level: usize,
    texels: *const u8,
  ) -> Result<(), B::Err> {
    B::set_texels(&self.raw, rect, mipmaps, level, texels)
  }

  pub fn clear(&self, rect: Rect, mipmaps: bool, value: *const u8) -> Result<(), B::Err> {
    B::clear_texels(&self.raw, rect, mipmaps, value)
  }
}
