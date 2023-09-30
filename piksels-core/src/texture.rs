use piksels_backend::{
  texture::{Rect, Size},
  Backend,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Texture<B>
where
  B: Backend,
{
  pub(crate) raw: B::Texture,
}

impl<B> Drop for Texture<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_texture(&self.raw);
  }
}

impl<B> Texture<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::Texture) -> Self {
    Self { raw }
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
