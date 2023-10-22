use piksels_backend::{
  blending::BlendingMode,
  color::RGBA32F,
  depth_stencil::{DepthTest, DepthWrite, StencilTest},
  face_culling::FaceCulling,
  scissor::Scissor,
  viewport::Viewport,
  Backend,
};

use crate::{
  render_targets::RenderTargets,
  shader::{Shader, TextureBindingPoint, Uniform, UniformBuffer, UniformBufferBindingPoint},
  texture::Texture,
};

#[derive(Debug)]
pub struct CmdBuf<B>
where
  B: Backend,
{
  pub(crate) raw: B::CmdBuf,
}

impl<B> CmdBuf<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::CmdBuf) -> Self {
    Self { raw }
  }

  pub fn blending(&self, value: BlendingMode) -> Result<&Self, B::Err> {
    B::cmd_buf_blending(&self.raw, value)?;
    Ok(self)
  }

  pub fn depth_test(&self, value: DepthTest) -> Result<&Self, B::Err> {
    B::cmd_buf_depth_test(&self.raw, value)?;
    Ok(self)
  }

  pub fn depth_write(&self, value: DepthWrite) -> Result<&Self, B::Err> {
    B::cmd_buf_depth_write(&self.raw, value)?;
    Ok(self)
  }

  pub fn stencil_test(&self, value: StencilTest) -> Result<&Self, B::Err> {
    B::cmd_buf_stencil_test(&self.raw, value)?;
    Ok(self)
  }

  pub fn face_culling(&self, value: FaceCulling) -> Result<&Self, B::Err> {
    B::cmd_buf_face_culling(&self.raw, value)?;
    Ok(self)
  }

  pub fn viewport(&self, value: Viewport) -> Result<&Self, B::Err> {
    B::cmd_buf_viewport(&self.raw, value)?;
    Ok(self)
  }

  pub fn scissor(&self, value: Scissor) -> Result<&Self, B::Err> {
    B::cmd_buf_scissor(&self.raw, value)?;
    Ok(self)
  }

  pub fn clear_color(&self, value: RGBA32F) -> Result<&Self, B::Err> {
    B::cmd_buf_clear_color(&self.raw, value)?;
    Ok(self)
  }

  pub fn clear_depth(&self, value: f32) -> Result<&Self, B::Err> {
    B::cmd_buf_clear_depth(&self.raw, value)?;
    Ok(self)
  }

  pub fn srgb(&self, value: bool) -> Result<&Self, B::Err> {
    B::cmd_buf_srgb(&self.raw, value)?;
    Ok(self)
  }

  pub fn uniform(&self, uniform: &Uniform<B>, value: *const u8) -> Result<&Self, B::Err> {
    B::cmd_buf_set_uniform(&self.raw, &uniform.raw, value)?;
    Ok(self)
  }

  /// Mark a texture as being active.
  pub fn texture(&self, texture: &Texture<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_texture(&self.raw, &texture.raw)?;
    Ok(self)
  }

  /// Connect a texture to a texture binding point.
  pub fn texture_binding_point(
    &self,
    texture: &Texture<B>,
    binding_point: &TextureBindingPoint<B>,
  ) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_texture_binding_point(&self.raw, &texture.raw, &binding_point.raw)?;
    Ok(self)
  }

  /// Mark a uniform buffer as being active.
  pub fn uniform_buffer(&self, uniform_buffer: &UniformBuffer<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_uniform_buffer(&self.raw, &uniform_buffer.raw)?;
    Ok(self)
  }

  /// Connect a uniform buffer unit to a uniform buffer binding point.
  pub fn uniform_buffer_binding_point(
    &self,
    uniform_buffer: &UniformBuffer<B>,
    binding_point: &UniformBufferBindingPoint<B>,
  ) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_uniform_buffer_binding_point(
      &self.raw,
      &uniform_buffer.raw,
      &binding_point.raw,
    )?;
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
