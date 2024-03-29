use std::marker::PhantomData;

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
  shader::{Shader, Uniform, UniformBuffer},
  texture::Texture,
  units::{UnitBindingPoint, Units},
  vertex_array::VertexArray,
};

pub struct Parent<P, T> {
  _phantom: PhantomData<*const (P, T)>,
}

pub struct Layer<B, T>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
  texture_units: Units<B>,
  uniform_buffer_units: Units<B>,
  unused_stack: Vec<InUse<B>>,
  in_use_stack: Vec<InUse<B>>,
  in_use: InUse<B>,
  _phantom: PhantomData<*const T>,
}

impl<B> Layer<B, ()>
where
  B: Backend,
{
  pub(crate) fn from_cmd_buf(
    cmd_buf: B::CmdBuf,
    max_texture_units: B::Unit,
    max_uniform_buffer_units: B::Unit,
  ) -> Self {
    Self {
      cmd_buf,
      texture_units: Units::new(max_texture_units),
      uniform_buffer_units: Units::new(max_uniform_buffer_units),
      unused_stack: Vec::default(),
      in_use_stack: Vec::default(),
      in_use: InUse::default(),
      _phantom: PhantomData,
    }
  }
}

impl<B, T> Layer<B, T>
where
  B: Backend,
{
  fn change_type<Q>(self) -> Layer<B, Q> {
    Layer {
      cmd_buf: self.cmd_buf,
      texture_units: self.texture_units,
      uniform_buffer_units: self.uniform_buffer_units,
      unused_stack: self.unused_stack,
      in_use_stack: self.in_use_stack,
      in_use: self.in_use,
      _phantom: PhantomData,
    }
  }

  fn deeper<Q>(mut self) -> Layer<B, Parent<Self, Q>> {
    let in_use = std::mem::replace(
      &mut self.in_use,
      self.unused_stack.pop().unwrap_or_default(),
    );
    self.in_use_stack.push(in_use);
    self.change_type()
  }

  pub fn group(self) -> Layer<B, Parent<Self, ()>> {
    self.deeper()
  }

  // TODO: I think we might need to put most of those functions under Layer<B, RenderTargets~>?
  pub fn blending(self, blending: BlendingMode) -> Result<Self, B::Err> {
    B::cmd_buf_blending(&self.cmd_buf, blending)?;
    Ok(self)
  }

  pub fn depth_test(self, depth_test: DepthTest) -> Result<Self, B::Err> {
    B::cmd_buf_depth_test(&self.cmd_buf, depth_test)?;
    Ok(self)
  }

  pub fn depth_write(self, depth_write: DepthWrite) -> Result<Self, B::Err> {
    B::cmd_buf_depth_write(&self.cmd_buf, depth_write)?;
    Ok(self)
  }

  pub fn stencil_test(self, stencil_test: StencilTest) -> Result<Self, B::Err> {
    B::cmd_buf_stencil_test(&self.cmd_buf, stencil_test)?;
    Ok(self)
  }

  pub fn face_culling(self, face_culling: FaceCulling) -> Result<Self, B::Err> {
    B::cmd_buf_face_culling(&self.cmd_buf, face_culling)?;
    Ok(self)
  }

  pub fn viewport(self, viewport: Viewport) -> Result<Self, B::Err> {
    B::cmd_buf_viewport(&self.cmd_buf, viewport)?;
    Ok(self)
  }

  pub fn scissor(self, scissor: Scissor) -> Result<Self, B::Err> {
    B::cmd_buf_scissor(&self.cmd_buf, scissor)?;
    Ok(self)
  }

  pub fn clear_color(self, clear_color: impl Into<Option<RGBA>>) -> Result<Self, B::Err> {
    B::cmd_buf_clear_color(&self.cmd_buf, clear_color.into())?;
    Ok(self)
  }

  pub fn clear_depth(self, clear_depth: impl Into<Option<f32>>) -> Result<Self, B::Err> {
    B::cmd_buf_clear_depth(&self.cmd_buf, clear_depth.into())?;
    Ok(self)
  }

  pub fn srgb(self, srgb: bool) -> Result<Self, B::Err> {
    B::cmd_buf_srgb(&self.cmd_buf, srgb)?;
    Ok(self)
  }

  pub fn texture(mut self, texture: &Texture<B>) -> Result<Self, B::Err> {
    let ubp = self.texture_units.get_unit()?;

    B::cmd_buf_bind_texture(&self.cmd_buf, &texture.raw, &ubp.unit)?;
    self.in_use.textures.push(ubp);

    Ok(self)
  }

  pub fn uniform_buffer(mut self, uniform_buffer: &UniformBuffer<B>) -> Result<Self, B::Err> {
    let ubp = self.uniform_buffer_units.get_unit()?;

    B::cmd_buf_bind_uniform_buffer(&self.cmd_buf, &uniform_buffer.raw, &ubp.unit)?;
    self.in_use.uniform_buffers.push(ubp);

    Ok(self)
  }
}

impl<B, P, T> Layer<B, Parent<P, T>>
where
  B: Backend,
{
  pub fn done(mut self) -> Layer<B, P> {
    self.mark_idle_and_clear();

    self.unused_stack.push(self.in_use);
    self.in_use = self.in_use_stack.pop().unwrap_or_default();

    self.change_type()
  }

  fn mark_idle_and_clear(&mut self) {
    self.mark_textures_idle();
    self.in_use.textures.clear();

    self.mark_uniform_buffers_idle();
    self.in_use.uniform_buffers.clear();
  }

  fn mark_textures_idle(&mut self) {
    for ubp in &self.in_use.textures {
      if let Some(ref scarce_index) = ubp.current_scarce_index {
        self
          .texture_units
          .idle(ubp.unit.clone(), scarce_index.clone());
      }
    }
  }

  fn mark_uniform_buffers_idle(&mut self) {
    for ubp in &self.in_use.uniform_buffers {
      if let Some(ref scarce_index) = ubp.current_scarce_index {
        self
          .uniform_buffer_units
          .idle(ubp.unit.clone(), scarce_index.clone());
      }
    }
  }
}

pub trait LayerTop<B>: Sized
where
  B: Backend,
{
  fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<Layer<B, Parent<Self, RenderTargets<B>>>, B::Err>;
}

impl<B> LayerTop<B> for Layer<B, ()>
where
  B: Backend,
{
  fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<Layer<B, Parent<Self, RenderTargets<B>>>, B::Err> {
    B::cmd_buf_bind_render_targets(&self.cmd_buf, &render_targets.raw)?;
    Ok(self.deeper())
  }
}

impl<B, L> LayerTop<B> for Layer<B, Layer<B, L>>
where
  B: Backend,
  L: LayerTop<B>,
{
  fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<Layer<B, Parent<Self, RenderTargets<B>>>, <B as Backend>::Err> {
    B::cmd_buf_bind_render_targets(&self.cmd_buf, &render_targets.raw)?;
    Ok(self.deeper())
  }
}

pub trait LayerRenderTargets<B>: Sized
where
  B: Backend,
{
  fn shader(self, shader: &Shader<B>) -> Result<Layer<B, Parent<Self, ShaderLayer<B>>>, B::Err>;
}

impl<B> LayerRenderTargets<B> for Layer<B, RenderTargets<B>>
where
  B: Backend,
{
  fn shader(
    self,
    shader: &Shader<B>,
  ) -> Result<Layer<B, Parent<Self, ShaderLayer<B>>>, <B as Backend>::Err> {
    B::cmd_buf_bind_shader(&self.cmd_buf, &shader.raw)?;
    Ok(self.deeper())
  }
}

impl<B, L> LayerRenderTargets<B> for Layer<B, Layer<B, L>>
where
  B: Backend,
  L: LayerRenderTargets<B>,
{
  fn shader(
    self,
    shader: &Shader<B>,
  ) -> Result<Layer<B, Parent<Self, ShaderLayer<B>>>, <B as Backend>::Err> {
    B::cmd_buf_bind_shader(&self.cmd_buf, &shader.raw)?;
    Ok(self.deeper())
  }
}

pub trait LayerShader<B>: Sized
where
  B: Backend,
{
  fn uniform(self, uniform: &Uniform<B>, value: *const u8) -> Result<Self, B::Err>;
  fn draw(self, vertex_array: &VertexArray<B>) -> Result<Self, B::Err>;
}

impl<B> LayerShader<B> for Layer<B, Shader<B>>
where
  B: Backend,
{
  fn uniform(self, uniform: &Uniform<B>, value: *const u8) -> Result<Self, <B as Backend>::Err> {
    B::cmd_buf_set_uniform(&self.cmd_buf, &uniform.raw, value)?;
    Ok(self)
  }

  fn draw(self, vertex_array: &VertexArray<B>) -> Result<Self, <B as Backend>::Err> {
    B::cmd_buf_draw_vertex_array(&self.cmd_buf, &vertex_array.raw)?;
    Ok(self)
  }
}

impl<B, L> LayerShader<B> for Layer<B, Layer<B, L>>
where
  B: Backend,
  L: LayerShader<B>,
{
  fn uniform(self, uniform: &Uniform<B>, value: *const u8) -> Result<Self, <B as Backend>::Err> {
    B::cmd_buf_set_uniform(&self.cmd_buf, &uniform.raw, value)?;
    Ok(self)
  }

  fn draw(self, vertex_array: &VertexArray<B>) -> Result<Self, <B as Backend>::Err> {
    B::cmd_buf_draw_vertex_array(&self.cmd_buf, &vertex_array.raw)?;
    Ok(self)
  }
}

// ================ OLD CODE

pub trait ChangeLayer<B>
where
  B: Backend,
{
  fn change_layer(
    cmd_buf: B::CmdBuf,
    texture_units: Units<B>,
    uniform_buffer_units: Units<B>,
    in_use_stack: Vec<InUse<B>>,
  ) -> Self;
}

#[derive(Debug)]
pub struct Layers<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
  texture_units: Units<B>,
  uniform_buffer_units: Units<B>,
  in_use_stack: Vec<InUse<B>>,
}

impl<B> ChangeLayer<B> for Layers<B>
where
  B: Backend,
{
  fn change_layer(
    cmd_buf: B::CmdBuf,
    texture_units: Units<B>,
    uniform_buffer_units: Units<B>,
    in_use_stack: Vec<InUse<B>>,
  ) -> Self {
    Self {
      cmd_buf,
      texture_units,
      uniform_buffer_units,
      in_use_stack,
    }
  }
}

impl<B> Layers<B>
where
  B: Backend,
{
  pub(crate) fn from_cmd_buf(
    cmd_buf: B::CmdBuf,
    max_texture_units: B::Unit,
    max_uniform_buffer_units: B::Unit,
  ) -> Result<Self, B::Err> {
    Ok(Self {
      cmd_buf,
      texture_units: Units::new(max_texture_units),
      uniform_buffer_units: Units::new(max_uniform_buffer_units),
      in_use_stack: Vec::default(),
    })
  }

  pub fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<RenderTargetsLayer<B>, B::Err> {
    B::cmd_buf_bind_render_targets(&self.cmd_buf, &render_targets.raw)?;

    Ok(RenderTargetsLayer::change_layer(
      self.cmd_buf,
      self.texture_units,
      self.uniform_buffer_units,
      self.in_use_stack,
    ))
  }

  pub fn done(&self) -> Result<(), B::Err> {
    B::cmd_buf_finish(&self.cmd_buf)
  }
}

#[derive(Debug)]
pub struct RenderTargetsLayer<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
  texture_units: Units<B>,
  uniform_buffer_units: Units<B>,
  in_use_stack: Vec<InUse<B>>,
}

impl<B> ChangeLayer<B> for RenderTargetsLayer<B>
where
  B: Backend,
{
  fn change_layer(
    cmd_buf: B::CmdBuf,
    texture_units: Units<B>,
    uniform_buffer_units: Units<B>,
    in_use_stack: Vec<InUse<B>>,
  ) -> Self {
    Self {
      cmd_buf,
      texture_units,
      uniform_buffer_units,
      in_use_stack,
    }
  }
}

impl<B> RenderTargetsLayer<B>
where
  B: Backend,
{
  pub fn shader(self, shader: &Shader<B>) -> Result<ShaderLayer<B>, B::Err> {
    B::cmd_buf_bind_shader(&self.cmd_buf, &shader.raw)?;
    Ok(ShaderLayer::change_layer(
      self.cmd_buf,
      self.texture_units,
      self.uniform_buffer_units,
      self.in_use_stack,
    ))
  }

  pub fn done(self) -> Layers<B> {
    Layers::change_layer(
      self.cmd_buf,
      self.texture_units,
      self.uniform_buffer_units,
      self.in_use_stack,
    )
  }
}

#[derive(Debug)]
pub struct ShaderLayer<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
  texture_units: Units<B>,
  uniform_buffer_units: Units<B>,
  in_use_stack: Vec<InUse<B>>,
}

impl<B> ChangeLayer<B> for ShaderLayer<B>
where
  B: Backend,
{
  fn change_layer(
    cmd_buf: B::CmdBuf,
    texture_units: Units<B>,
    uniform_buffer_units: Units<B>,
    in_use_stack: Vec<InUse<B>>,
  ) -> Self {
    Self {
      cmd_buf,
      texture_units,
      uniform_buffer_units,
      in_use_stack,
    }
  }
}

impl<B> ShaderLayer<B>
where
  B: Backend,
{
  pub fn uniform(self, uniform: &Uniform<B>, value: *const u8) -> Result<Self, B::Err> {
    B::cmd_buf_set_uniform(&self.cmd_buf, &uniform.raw, value)?;
    Ok(self)
  }

  pub fn draw(self, vertex_array: &VertexArray<B>) -> Result<Self, B::Err> {
    B::cmd_buf_draw_vertex_array(&self.cmd_buf, &vertex_array.raw)?;
    Ok(self)
  }

  pub fn done(self) -> RenderTargetsLayer<B> {
    RenderTargetsLayer::change_layer(
      self.cmd_buf,
      self.texture_units,
      self.uniform_buffer_units,
      self.in_use_stack,
    )
  }
}

#[derive(Debug)]
pub struct InUse<B>
where
  B: Backend,
{
  textures: Vec<UnitBindingPoint<B>>,
  uniform_buffers: Vec<UnitBindingPoint<B>>,
}

impl<B> Default for InUse<B>
where
  B: Backend,
{
  fn default() -> Self {
    Self {
      textures: Vec::default(),
      uniform_buffers: Vec::default(),
    }
  }
}

#[derive(Debug)]
pub struct GroupLayer<B, Parent>
where
  B: Backend,
  Parent: ?Sized,
{
  cmd_buf: B::CmdBuf,
  texture_units: Units<B>,
  uniform_buffer_units: Units<B>,
  in_use: InUse<B>,
  in_use_stack: Vec<InUse<B>>,
  _phantom: PhantomData<*const Parent>,
}

impl<B, Parent> ChangeLayer<B> for GroupLayer<B, Parent>
where
  B: Backend,
{
  fn change_layer(
    cmd_buf: B::CmdBuf,
    texture_units: Units<B>,
    uniform_buffer_units: Units<B>,
    mut in_use_stack: Vec<InUse<B>>,
  ) -> Self {
    let in_use = in_use_stack.pop().unwrap_or_default();

    Self {
      cmd_buf,
      texture_units,
      uniform_buffer_units,
      in_use,
      in_use_stack,
      _phantom: PhantomData,
    }
  }
}

impl<B, Parent> GroupLayer<B, Parent>
where
  B: Backend,
  Parent: ChangeLayer<B>,
{
  pub fn done(mut self) -> Parent {
    self.mark_idle_and_clear();
    self.in_use_stack.push(self.in_use);

    Parent::change_layer(
      self.cmd_buf,
      self.texture_units,
      self.uniform_buffer_units,
      self.in_use_stack,
    )
  }

  fn mark_idle_and_clear(&mut self) {
    self.mark_textures_idle();
    self.in_use.textures.clear();

    self.mark_uniform_buffers_idle();
    self.in_use.uniform_buffers.clear();
  }

  fn mark_textures_idle(&mut self) {
    for ubp in &self.in_use.textures {
      if let Some(ref scarce_index) = ubp.current_scarce_index {
        self
          .texture_units
          .idle(ubp.unit.clone(), scarce_index.clone());
      }
    }
  }

  fn mark_uniform_buffers_idle(&mut self) {
    for ubp in &self.in_use.uniform_buffers {
      if let Some(ref scarce_index) = ubp.current_scarce_index {
        self
          .uniform_buffer_units
          .idle(ubp.unit.clone(), scarce_index.clone());
      }
    }
  }
}

impl<B, Parent> GroupLayer<B, Parent>
where
  B: Backend,
{
  pub fn texture(mut self, texture: &Texture<B>) -> Result<Self, B::Err> {
    let ubp = self.texture_units.get_unit()?;

    B::cmd_buf_bind_texture(&self.cmd_buf, &texture.raw, &ubp.unit)?;
    self.in_use.textures.push(ubp);

    Ok(self)
  }

  pub fn uniform_buffer(mut self, uniform_buffer: &UniformBuffer<B>) -> Result<Self, B::Err> {
    let ubp = self.uniform_buffer_units.get_unit()?;

    B::cmd_buf_bind_uniform_buffer(&self.cmd_buf, &uniform_buffer.raw, &ubp.unit)?;
    self.in_use.uniform_buffers.push(ubp);

    Ok(self)
  }
}

/// Operations common to all layers.
pub trait LayerCommons<B>: Sized
where
  B: Backend,
{
  fn blending(self, blending: BlendingMode) -> Result<Self, B::Err>;
  fn depth_test(self, depth_test: DepthTest) -> Result<Self, B::Err>;
  fn depth_write(self, depth_write: DepthWrite) -> Result<Self, B::Err>;
  fn stencil_test(self, stencil_test: StencilTest) -> Result<Self, B::Err>;
  fn face_culling(self, face_culling: FaceCulling) -> Result<Self, B::Err>;
  fn viewport(self, viewport: Viewport) -> Result<Self, B::Err>;
  fn scissor(self, scissor: Scissor) -> Result<Self, B::Err>;
  fn clear_color(self, clear_color: impl Into<Option<RGBA>>) -> Result<Self, B::Err>;
  fn clear_depth(self, clear_depth: impl Into<Option<f32>>) -> Result<Self, B::Err>;
  fn srgb(self, srgb: bool) -> Result<Self, B::Err>;
  fn group(self) -> GroupLayer<B, Self>;
}

macro_rules! impl_layer_variables {
  ($($ty:ident),* $(,)?) => {
    $(
      impl<B> LayerCommons<B> for $ty<B>
      where
        B: Backend,
      {
        fn blending(self, blending: BlendingMode) -> Result<Self, B::Err> {
          B::cmd_buf_blending(&self.cmd_buf, blending)?;
          Ok(self)
        }

        fn depth_test(self, depth_test: DepthTest) -> Result<Self, B::Err> {
          B::cmd_buf_depth_test(&self.cmd_buf, depth_test)?;
          Ok(self)
        }

        fn depth_write(self, depth_write: DepthWrite) -> Result<Self, B::Err> {
          B::cmd_buf_depth_write(&self.cmd_buf, depth_write)?;
          Ok(self)
        }

        fn stencil_test(self, stencil_test: StencilTest) -> Result<Self, B::Err> {
          B::cmd_buf_stencil_test(&self.cmd_buf, stencil_test)?;
          Ok(self)
        }

        fn face_culling(self, face_culling: FaceCulling) -> Result<Self, B::Err> {
          B::cmd_buf_face_culling(&self.cmd_buf, face_culling)?;
          Ok(self)
        }

        fn viewport(self, viewport: Viewport) -> Result<Self, B::Err> {
          B::cmd_buf_viewport(&self.cmd_buf, viewport)?;
          Ok(self)
        }

        fn scissor(self, scissor: Scissor) -> Result<Self, B::Err> {
          B::cmd_buf_scissor(&self.cmd_buf, scissor)?;
          Ok(self)
        }

        fn clear_color(self, clear_color: impl Into<Option<RGBA>>) -> Result<Self, B::Err> {
          B::cmd_buf_clear_color(&self.cmd_buf, clear_color.into())?;
          Ok(self)
        }

        fn clear_depth(self, clear_depth: impl Into<Option<f32>>) -> Result<Self, B::Err> {
          B::cmd_buf_clear_depth(&self.cmd_buf, clear_depth.into())?;
          Ok(self)
        }

        fn srgb(self, srgb: bool) -> Result<Self, B::Err> {
          B::cmd_buf_srgb(&self.cmd_buf, srgb)?;
          Ok(self)
        }

        fn group(self) -> GroupLayer<B, Self> {
          GroupLayer::change_layer(self.cmd_buf, self.texture_units, self.uniform_buffer_units, self.in_use_stack)
        }
      }
    )*
  };
}

impl_layer_variables!(Layers, RenderTargetsLayer, ShaderLayer);
