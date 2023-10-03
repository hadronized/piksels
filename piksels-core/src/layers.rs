use piksels_backend::Backend;

use crate::{render_targets::RenderTargets, shader::Shader, vertex_array::VertexArray};

#[derive(Debug)]
pub struct Layers<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
}

impl<B> Layers<B>
where
  B: Backend,
{
  pub(crate) fn from_cmd_buf(cmd_buf: B::CmdBuf) -> Self {
    Self { cmd_buf }
  }

  pub fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<RenderTargetsLayer<B>, B::Err> {
    B::cmd_buf_bind_render_targets(&self.cmd_buf, &render_targets.raw)?;
    Ok(RenderTargetsLayer::from_cmd_buf(self.cmd_buf))
  }

  pub fn finish(&self) -> Result<(), B::Err> {
    B::cmd_buf_finish(&self.cmd_buf)
  }
}

#[derive(Debug)]
pub struct RenderTargetsLayer<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
}

impl<B> RenderTargetsLayer<B>
where
  B: Backend,
{
  fn from_cmd_buf(cmd_buf: B::CmdBuf) -> Self {
    Self { cmd_buf }
  }

  pub fn shader(self, shader: &Shader<B>) -> Result<ShaderLayer<B>, B::Err> {
    B::cmd_buf_bind_shader(&self.cmd_buf, &shader.raw)?;
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
  cmd_buf: B::CmdBuf,
}

impl<B> ShaderLayer<B>
where
  B: Backend,
{
  fn from_cmd_buf(cmd_buf: B::CmdBuf) -> Self {
    Self { cmd_buf }
  }

  pub fn draw(&self, vertex_array: &VertexArray<B>) -> Result<(), B::Err> {
    B::cmd_buf_draw_vertex_array(&self.cmd_buf, &vertex_array.raw)
  }

  pub fn finish(self) -> RenderTargetsLayer<B> {
    RenderTargetsLayer::from_cmd_buf(self.cmd_buf)
  }
}
