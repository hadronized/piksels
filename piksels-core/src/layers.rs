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

/// Variables that can be changed through every part of a [`Layers`] object and its children.
pub trait LayerVariable<B>
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
  // TODO: binding resources
}

macro_rules! impl_layer_variables {
  ($($ty:ident),* $(,)?) => {
    $(
      impl<B> LayerVariable<B> for $ty<B>
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
      }
    )*
  };
}

impl_layer_variables!(Layers, RenderTargetsLayer, ShaderLayer);
