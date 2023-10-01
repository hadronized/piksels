use piksels_backend::Backend;

use crate::{pipeline::CmdBuf, render_targets::RenderTargets};

#[derive(Debug)]
pub struct RenderTargetsLayer<'a, B>
where
  B: Backend,
{
  cmd_buf: CmdBuf<B>,
  render_targets_raw: &'a B::RenderTargets,
}

impl<'a, B> RenderTargetsLayer<'a, B>
where
  B: Backend,
{
  pub(crate) fn new(cmd_buf: CmdBuf<B>, render_targets: &'a RenderTargets<B>) -> Self {
    let render_targets_raw = &render_targets.raw;

    Self {
      cmd_buf,
      render_targets_raw,
    }
  }
}
