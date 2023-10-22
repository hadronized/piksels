use std::sync::{Arc, Mutex, Weak};

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
  cache::Cache,
  render_targets::RenderTargets,
  shader::{
    Shader, TextureBindingPoint, Uniform, UniformBuffer, UniformBufferBindingPoint,
    UniformBufferUnit,
  },
  texture::{Texture, TextureUnit},
};

#[derive(Debug)]
pub struct CmdBuf<B>
where
  B: Backend,
{
  pub(crate) raw: B::CmdBuf,
  cache: Weak<Mutex<Cache<B>>>,
}

impl<B> Drop for CmdBuf<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    if let Some(Ok(mut cache)) = self.cache.upgrade().map(|c| c.lock()) {
      cache.untrack_cmd_buf(&self.raw);
    }
  }
}

/// Creates methods of the form:
///
///   pub $name(&self, value: $ty) -> Result<&Self, B::Err>
///
/// The syntax is:
///
///  mk_cmd_buf_cached_methods!(
///    // first form
///    foo: FooType = cmd_buf_foo,
///
///    // second form, to add optional methods to the expression value
///    bar: BarType = cmd_buf_bar @value value.into(),
///  );
macro_rules! mk_cmd_buf_cached_method {
  ($name:ident : $ty:ty = $method:ident) => {
    pub fn $name(&self, value: &$ty) -> Result<&Self, B::Err> {
      self
        .cache
        .lock()
        .map_err(From::from)?
        .$name()
        .set_if_invalid(value, || B::$method(&self.raw, value.clone()))
        .map(move |_| self)
    }
  };
}

impl<B> CmdBuf<B>
where
  B: Backend,
{
  mk_cmd_buf_cached_method!(blending: BlendingMode = cmd_buf_blending);

  mk_cmd_buf_cached_method!(depth_test: DepthTest = cmd_buf_depth_test);

  mk_cmd_buf_cached_method!(depth_write: DepthWrite = cmd_buf_depth_write);

  mk_cmd_buf_cached_method!(stencil_test: StencilTest = cmd_buf_stencil_test);

  mk_cmd_buf_cached_method!(face_culling: FaceCulling = cmd_buf_face_culling);

  mk_cmd_buf_cached_method!(viewport: Viewport = cmd_buf_viewport);

  mk_cmd_buf_cached_method!(scissor: Scissor = cmd_buf_scissor);

  mk_cmd_buf_cached_method!(clear_color: RGBA32F = cmd_buf_clear_color);

  mk_cmd_buf_cached_method!(clear_depth: f32 = cmd_buf_clear_depth);

  mk_cmd_buf_cached_method!(srgb: bool = cmd_buf_srgb);

  pub(crate) fn from_raw(raw: B::CmdBuf, cache: Weak<Mutex<Cache<B>>>) -> Self {
    Self { raw, cache }
  }

  pub fn uniform(&self, uniform: &Uniform<B>, value: *const u8) -> Result<&Self, B::Err> {
    B::cmd_buf_set_uniform(&self.raw, &uniform.raw, value)?;
    Ok(self)
  }

  /// Mark a texture as being active.
  pub fn texture(&self, texture: &Texture<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_texture(&self.raw, &texture.raw, &unit.raw)?;
    Ok(self)
  }

  /// Connect a texture unit to a texture binding point.
  pub fn texture_binding_point(
    &self,
    binding_point: &TextureBindingPoint<B>,
  ) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_texture_unit(&self.raw, &unit.raw, &binding_point.raw)?;
    Ok(self)
  }

  /// Mark a uniform buffer as being active.
  pub fn uniform_buffer(&self, uniform_buffer: &UniformBuffer<B>) -> Result<&Self, B::Err> {
    B::cmd_buf_bind_uniform_buffer(&self.raw, &uniform_buffer.raw, &unit.raw)?;
    Ok(self)
  }

  /// Connect a uniform buffer unit to a uniform buffer binding point.
  pub fn uniform_buffer_binding_point(
    &self,
    uniform_buffer: &UniformBuffer<B>,
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
