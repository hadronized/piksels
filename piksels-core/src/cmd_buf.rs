use piksels_backend::{
  blending::BlendingMode,
  color::RGBA,
  depth_stencil::{DepthTest, DepthWrite, StencilTest},
  face_culling::FaceCulling,
  scissor::Scissor,
  viewport::Viewport,
  Backend,
};

use crate::{
  render_targets::RenderTargets,
  shader::{
    Shader, TextureBindingPoint, Uniform, UniformBuffer, UniformBufferBindingPoint,
    UniformBufferUnit,
  },
  texture::{Texture, TextureUnit},
};

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

  pub fn blending(&self, blending: BlendingMode) -> Result<&Self, B::Err> {
    B::cmd_buf_blending(&self.raw, blending)?;
    Ok(self)
  }

  pub fn depth_test(&self, depth_test: DepthTest) -> Result<&Self, B::Err> {
    B::cmd_buf_depth_test(&self.raw, depth_test)?;
    Ok(self)
  }

  pub fn depth_write(&self, depth_write: DepthWrite) -> Result<&Self, B::Err> {
    B::cmd_buf_depth_write(&self.raw, depth_write)?;
    Ok(self)
  }

  pub fn stencil_test(&self, stencil_test: StencilTest) -> Result<&Self, B::Err> {
    B::cmd_buf_stencil_test(&self.raw, stencil_test)?;
    Ok(self)
  }

  pub fn face_culling(&self, face_culling: FaceCulling) -> Result<&Self, B::Err> {
    B::cmd_buf_face_culling(&self.raw, face_culling)?;
    Ok(self)
  }

  pub fn viewport(&self, viewport: Viewport) -> Result<&Self, B::Err> {
    B::cmd_buf_viewport(&self.raw, viewport)?;
    Ok(self)
  }

  pub fn scissor(&self, scissor: Scissor) -> Result<&Self, B::Err> {
    B::cmd_buf_scissor(&self.raw, scissor)?;
    Ok(self)
  }

  pub fn clear_color(&self, clear_color: impl Into<Option<RGBA>>) -> Result<&Self, B::Err> {
    B::cmd_buf_clear_color(&self.raw, clear_color.into())?;
    Ok(self)
  }

  pub fn clear_depth(&self, clear_depth: impl Into<Option<f32>>) -> Result<&Self, B::Err> {
    B::cmd_buf_clear_depth(&self.raw, clear_depth.into())?;
    Ok(self)
  }

  pub fn srgb(&self, srgb: bool) -> Result<&Self, B::Err> {
    B::cmd_buf_srgb(&self.raw, srgb)?;
    Ok(self)
  }

  pub fn uniform(&self, uniform: &Uniform<B>, value: *const u8) -> Result<&Self, B::Err> {
    B::cmd_buf_set_uniform(&self.raw, &uniform.raw, value)?;
    Ok(self)
  }

  /// Mark a texture as being active.
  pub fn texture(&self, texture: &Texture<B>, unit: &TextureUnit<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_texture(&self.raw, &texture.raw, &unit.raw)?;
    Ok(self)
  }

  /// Connect a texture unit to a texture binding point.
  pub fn texture_binding_point(
    &self,
    unit: &TextureUnit<B>,
    binding_point: &TextureBindingPoint<B>,
  ) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_texture_unit(&self.raw, &unit.raw, &binding_point.raw)?;
    Ok(self)
  }

  /// Mark a uniform buffer as being active.
  pub fn uniform_buffer(
    &self,
    uniform_buffer: &UniformBuffer<B>,
    unit: &UniformBufferUnit<B>,
  ) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_uniform_buffer(&self.raw, &uniform_buffer.raw, &unit.raw)?;
    Ok(self)
  }

  /// Connect a uniform buffer unit to a uniform buffer binding point.
  pub fn uniform_buffer_binding_point(
    &self,
    unit: &UniformBufferUnit<B>,
    binding_point: &UniformBufferBindingPoint<B>,
  ) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_uniform_buffer_unit(&self.raw, &unit.raw, &binding_point.raw)?;
    Ok(self)
  }

  pub fn render_targets(&self, render_targets: &RenderTargets<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_render_targets(&self.raw, &render_targets.raw)?;
    Ok(self)
  }

  pub fn shader(&self, shader: &Shader<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_shader(&self.raw, &shader.raw)?;
    Ok(self)
  }

  pub fn finish(&self) -> Result<(), B::Err> {
    B::cmd_buf_finish(&self.raw)
  }
}
