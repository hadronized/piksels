use piksels_backend::{
  blending::BlendingMode,
  color::RGBA,
  depth_stencil::{DepthTest, DepthWrite, StencilTest},
  face_culling::FaceCulling,
  scissor::Scissor,
  viewport::Viewport,
  Backend,
};

use crate::{render_targets::RenderTargets, shader::Shader, vertex_array::VertexArray};

#[derive(Debug, Eq, PartialEq)]
pub struct CmdBuf<B>
where
  B: Backend,
{
  pub(crate) raw: B::CmdBuf,
}

impl<B> Drop for CmdBuf<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_cmd_buf(&self.raw);
  }
}

impl<B> CmdBuf<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::CmdBuf) -> Self {
    Self { raw }
  }

  pub fn blending(&self, blending: BlendingMode) -> Result<(), B::Err> {
    B::cmd_buf_blending(&self.raw, blending)
  }

  pub fn depth_test(&self, depth_test: DepthTest) -> Result<(), B::Err> {
    B::cmd_buf_depth_test(&self.raw, depth_test)
  }

  pub fn depth_write(&self, depth_write: DepthWrite) -> Result<(), B::Err> {
    B::cmd_buf_depth_write(&self.raw, depth_write)
  }

  pub fn stencil_test(&self, stencil_test: StencilTest) -> Result<(), B::Err> {
    B::cmd_buf_stencil_test(&self.raw, stencil_test)
  }

  pub fn cmd_buf_face_culling(&self, face_culling: FaceCulling) -> Result<(), B::Err> {
    B::cmd_buf_face_culling(&self.raw, face_culling)
  }

  pub fn cmd_buf_viewport(&self, viewport: Viewport) -> Result<(), B::Err> {
    B::cmd_buf_viewport(&self.raw, viewport)
  }

  pub fn cmd_buf_scissor(&self, scissor: Scissor) -> Result<(), B::Err> {
    B::cmd_buf_scissor(&self.raw, scissor)
  }

  pub fn cmd_buf_clear_color(&self, clear_color: Option<RGBA>) -> Result<(), B::Err> {
    B::cmd_buf_clear_color(&self.raw, clear_color)
  }

  pub fn cmd_buf_clear_depth(&self, clear_depth: Option<f32>) -> Result<(), B::Err> {
    B::cmd_buf_clear_depth(&self.raw, clear_depth)
  }

  pub fn cmd_buf_srgb(&self, srgb: bool) -> Result<(), B::Err> {
    B::cmd_buf_srgb(&self.raw, srgb)
  }

  pub fn cmd_buf_bind_render_targets(
    &self,
    render_targets: &RenderTargets<B>,
  ) -> Result<(), B::Err> {
    B::cmd_buf_bind_render_targets(&self.raw, &render_targets.raw)
  }

  pub fn cmd_buf_bind_shader(&self, shader: &Shader<B>) -> Result<(), B::Err> {
    B::cmd_buf_bind_shader(&self.raw, &shader.raw)
  }

  pub fn cmd_buf_draw_vertex_array(&self, vertex_array: &VertexArray<B>) -> Result<(), B::Err> {
    B::cmd_buf_draw_vertex_array(&self.raw, &vertex_array.raw)
  }
}
