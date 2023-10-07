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
  vertex_array::VertexArray,
};

pub trait ChangeLayer<B>
where
  B: Backend,
{
  fn change_layer(cmd_buf: B::CmdBuf, in_use_stack: Vec<GroupLayerInUse<B>>) -> Self;
}

#[derive(Debug)]
pub struct Layers<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
  in_use_stack: Vec<GroupLayerInUse<B>>,
}

impl<B> ChangeLayer<B> for Layers<B>
where
  B: Backend,
{
  fn change_layer(cmd_buf: B::CmdBuf, in_use_stack: Vec<GroupLayerInUse<B>>) -> Self {
    Self {
      cmd_buf,
      in_use_stack,
    }
  }
}

impl<B> Layers<B>
where
  B: Backend,
{
  pub(crate) fn from_cmd_buf(cmd_buf: B::CmdBuf) -> Self {
    Self {
      cmd_buf,
      in_use_stack: Vec::default(),
    }
  }

  pub fn render_targets(
    self,
    render_targets: &RenderTargets<B>,
  ) -> Result<RenderTargetsLayer<B>, B::Err> {
    B::cmd_buf_bind_render_targets(&self.cmd_buf, &render_targets.raw)?;

    Ok(RenderTargetsLayer::change_layer(
      self.cmd_buf,
      self.in_use_stack,
    ))
  }

  // TODO: clear / idle
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
  in_use_stack: Vec<GroupLayerInUse<B>>,
}

impl<B> ChangeLayer<B> for RenderTargetsLayer<B>
where
  B: Backend,
{
  fn change_layer(cmd_buf: B::CmdBuf, in_use_stack: Vec<GroupLayerInUse<B>>) -> Self {
    Self {
      cmd_buf,
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
    Ok(ShaderLayer::change_layer(self.cmd_buf, self.in_use_stack))
  }

  // TODO: in use / idle
  pub fn done(self) -> Layers<B> {
    Layers::change_layer(self.cmd_buf, self.in_use_stack)
  }
}

#[derive(Debug)]
pub struct ShaderLayer<B>
where
  B: Backend,
{
  cmd_buf: B::CmdBuf,
  in_use_stack: Vec<GroupLayerInUse<B>>,
}

impl<B> ChangeLayer<B> for ShaderLayer<B>
where
  B: Backend,
{
  fn change_layer(cmd_buf: B::CmdBuf, in_use_stack: Vec<GroupLayerInUse<B>>) -> Self {
    Self {
      cmd_buf,
      in_use_stack,
    }
  }
}

impl<B> ShaderLayer<B>
where
  B: Backend,
{
  pub fn set_uniform(&self, uniform: &Uniform<B>, value: *const u8) -> Result<(), B::Err> {
    B::cmd_buf_set_uniform(&self.cmd_buf, &uniform.raw, value)
  }

  pub fn draw(&self, vertex_array: &VertexArray<B>) -> Result<(), B::Err> {
    B::cmd_buf_draw_vertex_array(&self.cmd_buf, &vertex_array.raw)
  }

  // TODO: in use / idle
  pub fn done(self) -> RenderTargetsLayer<B> {
    RenderTargetsLayer::change_layer(self.cmd_buf, self.in_use_stack)
  }
}

#[derive(Debug)]
pub struct GroupLayerInUse<B>
where
  B: Backend,
{
  in_use_textures: Vec<B::Unit>,
  in_use_uniform_buffers: Vec<B::Unit>,
}

impl<B> Default for GroupLayerInUse<B>
where
  B: Backend,
{
  fn default() -> Self {
    Self {
      in_use_textures: Vec::default(),
      in_use_uniform_buffers: Vec::default(),
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
  in_use: GroupLayerInUse<B>,
  in_use_stack: Vec<GroupLayerInUse<B>>,
  _phantom: PhantomData<*const Parent>,
}

impl<B, Parent> ChangeLayer<B> for GroupLayer<B, Parent>
where
  B: Backend,
{
  fn change_layer(cmd_buf: B::CmdBuf, mut in_use_stack: Vec<GroupLayerInUse<B>>) -> Self {
    let in_use = in_use_stack.pop().unwrap_or_default();

    Self {
      cmd_buf,
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
  // TODO: in use / idle
  pub fn done(mut self) -> Parent {
    self.in_use_stack.push(self.in_use);
    Parent::change_layer(self.cmd_buf, self.in_use_stack)
  }
}

impl<B, Parent> GroupLayer<B, Parent>
where
  B: Backend,
{
  pub fn texture(&self, _texture: &Texture<B>) -> Result<(), B::Err> {
    todo!()
  }

  pub fn uniform_buffer(&self, _uniform_buffer: &UniformBuffer<B>) -> Result<(), B::Err> {
    todo!()
  }
}

/// Operations common to all layers.
pub trait LayerCommons<B>
where
  B: Backend,
{
  fn blending(&self, blending: BlendingMode) -> Result<(), B::Err>;
  fn depth_test(&self, depth_test: DepthTest) -> Result<(), B::Err>;
  fn depth_write(&self, depth_write: DepthWrite) -> Result<(), B::Err>;
  fn stencil_test(&self, stencil_test: StencilTest) -> Result<(), B::Err>;
  fn face_culling(&self, face_culling: FaceCulling) -> Result<(), B::Err>;
  fn viewport(&self, viewport: Viewport) -> Result<(), B::Err>;
  fn scissor(&self, scissor: Scissor) -> Result<(), B::Err>;
  fn clear_color(&self, clear_color: impl Into<Option<RGBA>>) -> Result<(), B::Err>;
  fn clear_depth(&self, clear_depth: impl Into<Option<f32>>) -> Result<(), B::Err>;
  fn srgb(&self, srgb: bool) -> Result<(), B::Err>;
  fn group(self) -> GroupLayer<B, Self>;
}

macro_rules! impl_layer_variables {
  ($($ty:ident),* $(,)?) => {
    $(
      impl<B> LayerCommons<B> for $ty<B>
      where
        B: Backend,
      {
        fn blending(&self, blending: BlendingMode) -> Result<(), B::Err> {
          B::cmd_buf_blending(&self.cmd_buf, blending)
        }

        fn depth_test(&self, depth_test: DepthTest) -> Result<(), B::Err> {
          B::cmd_buf_depth_test(&self.cmd_buf, depth_test)
        }

        fn depth_write(&self, depth_write: DepthWrite) -> Result<(), B::Err> {
          B::cmd_buf_depth_write(&self.cmd_buf, depth_write)
        }

        fn stencil_test(&self, stencil_test: StencilTest) -> Result<(), B::Err> {
          B::cmd_buf_stencil_test(&self.cmd_buf, stencil_test)
        }

        fn face_culling(&self, face_culling: FaceCulling) -> Result<(), B::Err> {
          B::cmd_buf_face_culling(&self.cmd_buf, face_culling)
        }

        fn viewport(&self, viewport: Viewport) -> Result<(), B::Err> {
          B::cmd_buf_viewport(&self.cmd_buf, viewport)
        }

        fn scissor(&self, scissor: Scissor) -> Result<(), B::Err> {
          B::cmd_buf_scissor(&self.cmd_buf, scissor)
        }

        fn clear_color(&self, clear_color: impl Into<Option<RGBA>>) -> Result<(), B::Err> {
          B::cmd_buf_clear_color(&self.cmd_buf, clear_color.into())
        }

        fn clear_depth(&self, clear_depth: impl Into<Option<f32>>) -> Result<(), B::Err> {
          B::cmd_buf_clear_depth(&self.cmd_buf, clear_depth.into())
        }

        fn srgb(&self, srgb: bool) -> Result<(), B::Err> {
          B::cmd_buf_srgb(&self.cmd_buf, srgb)
        }

        fn group(self) -> GroupLayer<B, Self> {
          GroupLayer::change_layer(self.cmd_buf, self.in_use_stack)
        }
      }
    )*
  };
}

impl_layer_variables!(Layers, RenderTargetsLayer, ShaderLayer);
