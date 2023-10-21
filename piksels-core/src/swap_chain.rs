use std::sync::{Mutex, Weak};

use piksels_backend::Backend;

use crate::{cache::ScarceCache, render_targets::RenderTargets};

#[derive(Debug)]
pub struct SwapChain<B>
where
  B: Backend,
{
  pub(crate) raw: B::SwapChain,
  cache: Weak<Mutex<ScarceCache<B>>>,
}

impl<B> Drop for SwapChain<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    if let Some(Ok(mut cache)) = self.cache.upgrade().map(|c| c.lock()) {
      cache.untrack_swap_chain(&self.raw);
    }
  }
}

impl<B> SwapChain<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::SwapChain, cache: Weak<Mutex<ScarceCache<B>>>) -> Self {
    Self { raw, cache }
  }

  pub fn render_targets(&self) -> Result<RenderTargets<B>, B::Err> {
    B::swap_chain_render_targets(&self.raw)
      .map(|raw| RenderTargets::from_raw(raw, self.cache.clone()))
  }

  pub fn present(&self, render_targets: &RenderTargets<B>) -> Result<(), B::Err> {
    B::present_render_targets(&self.raw, &render_targets.raw)
  }
}
