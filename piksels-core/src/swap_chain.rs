use piksels_backend::Backend;

use crate::render_targets::RenderTargets;

#[derive(Debug, Eq, PartialEq)]
pub struct SwapChain<B>
where
  B: Backend,
{
  pub(crate) raw: B::SwapChain,
}

impl<B> Drop for SwapChain<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_swap_chain(&self.raw);
  }
}

impl<B> SwapChain<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::SwapChain) -> Self {
    Self { raw }
  }

  pub fn render_targets(&self) -> Result<RenderTargets<B>, B::Err> {
    B::swap_chain_render_targets(&self.raw).map(RenderTargets::from_raw)
  }

  pub fn present(&self, render_targets: &RenderTargets<B>) -> Result<(), B::Err> {
    B::present_render_targets(&self.raw, &render_targets.raw)
  }
}
