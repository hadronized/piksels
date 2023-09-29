use crate::{
  blending::BlendingMode,
  color::RGBA,
  depth_stencil::{DepthTest, DepthWrite, StencilTest},
  face_culling::FaceCulling,
  scissor::Scissor,
  viewport::Viewport,
};

/// Pipeline command that can be run anywhere in through the pipeline layers.
#[derive(Clone, Debug, PartialEq)]
pub enum CommonCmd {
  /// Set blending for the next pipeline commands.
  Blending(BlendingMode),

  /// Set depth test for the next pipeline commands.
  DepthTest(DepthTest),

  /// Set depth write for the next pipeline commands.
  DepthWrite(DepthWrite),

  /// Set stencil test for the next pipeline commands.
  StencilTest(StencilTest),

  /// Set face culling for the next pipeline commands.
  FaceCulling(FaceCulling),

  /// Set viewport for the next pipeline commands.
  Viewport(Viewport),

  /// Set scissor for the next pipeline commands.
  Scissor(Scissor),

  /// Color to use when clearing color buffers.
  ///
  /// Set to [`None`] to leave color buffers untouched.
  ClearColor(Option<RGBA>),

  /// Depth value to use when clearing the depth buffer.
  ///
  /// Set to [`None`] to leave the depth buffer untouched.
  ClearDepth(Option<f32>),

  /// Enable sRGB support.
  SRGB(bool),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RenderTargetsCmd {
  BindRenderTargets { handle: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShaderCmd {
  BindShader { handle: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VertexArrayCmd {
  RenderVertexArray { handle: usize },
}
