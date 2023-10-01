use piksels_backend::Backend;

use crate::{
  pipeline::CmdBuf, render_targets::RenderTargets, shader::Shader, vertex_array::VertexArray,
};

#[derive(Debug)]
pub struct Layers<B>
where
  B: Backend,
{
  cmd_buf: CmdBuf<B>,
}

impl<B> Layers<B>
where
  B: Backend,
{
  pub(crate) fn from_cmd_buf(cmd_buf: CmdBuf<B>) -> Self {
    Self { cmd_buf }
  }

  pub fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<RenderTargetsLayer<B>, B::Err> {
    self.cmd_buf.bind_render_targets(render_targets)?;
    Ok(RenderTargetsLayer::from_cmd_buf(self.cmd_buf))
  }

  // TODO: do we really return Layers directly, or something to wait on like a Frame or something?
  pub fn finish(&self) -> Result<(), B::Err> {
    self.cmd_buf.finish()
  }
}

#[derive(Debug)]
pub struct RenderTargetsLayer<B>
where
  B: Backend,
{
  cmd_buf: CmdBuf<B>,
}

impl<B> RenderTargetsLayer<B>
where
  B: Backend,
{
  fn from_cmd_buf(cmd_buf: CmdBuf<B>) -> Self {
    Self { cmd_buf }
  }

  pub fn shader(self, shader: &Shader<B>) -> Result<ShaderLayer<B>, B::Err> {
    self.cmd_buf.bind_shader(shader)?;
    Ok(ShaderLayer::from_cmd_buf(self.cmd_buf))
  }

  pub fn finish(self) -> Layers<B> {
    Layers::from_cmd_buf(self.cmd_buf)
  }
}

#[derive(Debug)]
pub struct ShaderLayer<B>
where
  B: Backend,
{
  cmd_buf: CmdBuf<B>,
}

impl<B> ShaderLayer<B>
where
  B: Backend,
{
  fn from_cmd_buf(cmd_buf: CmdBuf<B>) -> Self {
    Self { cmd_buf }
  }

  pub fn draw(&self, vertex_array: &VertexArray<B>) -> Result<(), B::Err> {
    self.cmd_buf.cmd_buf_draw_vertex_array(vertex_array)
  }

  pub fn finish(self) -> RenderTargetsLayer<B> {
    RenderTargetsLayer::from_cmd_buf(self.cmd_buf)
  }
}
