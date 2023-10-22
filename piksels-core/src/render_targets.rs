use std::sync::{Mutex, Weak};

use piksels_backend::Backend;

use crate::cache::Cache;

#[derive(Debug)]
pub struct RenderTargets<B>
where
  B: Backend,
{
  pub(crate) raw: B::RenderTargets,
  cache: Weak<Mutex<Cache<B>>>,
}

impl<B> Drop for RenderTargets<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    if let Some(Ok(mut cache)) = self.cache.upgrade().map(|c| c.lock()) {
      cache.untrack_render_targets(&self.raw);
    }
  }
}

impl<B> RenderTargets<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::RenderTargets, cache: Weak<Mutex<Cache<B>>>) -> Self {
    Self { raw, cache }
  }

  pub fn color_attachment(&self, index: usize) -> Result<ColorAttachment<B>, B::Err> {
    B::get_color_attachment(&self.raw, index).map(|raw| ColorAttachment { raw })
  }

  pub fn depth_stencil_attachment(
    &self,
    index: usize,
  ) -> Result<DepthStencilAttachment<B>, B::Err> {
    B::get_depth_stencil_attachment(&self.raw, index).map(|raw| DepthStencilAttachment { raw })
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorAttachment<B>
where
  B: Backend,
{
  pub(crate) raw: B::ColorAttachment,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DepthStencilAttachment<B>
where
  B: Backend,
{
  pub(crate) raw: B::DepthStencilAttachment,
}
