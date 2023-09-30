use std::collections::HashSet;

use blending::BlendingMode;
use color::RGBA;
use depth_stencil::{DepthTest, DepthWrite, StencilTest};
use face_culling::FaceCulling;
use pipeline::CmdBuf;
use scissor::Scissor;
use texture::{Texture, TextureSampling};
use viewport::Viewport;

use crate::{
  render_targets::{ColorAttachment, DepthStencilAttachment, RenderTargets},
  shader::{Shader, ShaderSources, Uniform, UniformBuffer, UniformType},
  vertex_array::{VertexArray, VertexArrayData, VertexArrayUpdate},
};

/// A macro to help creating backend types methods.
///
/// Such a rule will automatically create some common methods.
macro_rules! mk_bckd_type_getters {
  ($ty:ty, $($method_name:ident -> $method_ret:ty ),+) => {
    impl $ty {
      $(
        pub fn $method_name(&self) -> $method_ret {
          self.$method_name
        }
      )+
    }
  };
}

pub mod blending;
pub mod color;
pub mod depth_stencil;
pub mod face_culling;
pub mod pipeline;
pub mod pixel;
pub mod primitive;
pub mod render_targets;
pub mod scissor;
pub mod shader;
pub mod texture;
pub mod vertex;
pub mod vertex_array;
pub mod viewport;

pub trait Backend {
  type Err;

  /// Backend author.
  fn author(&self) -> Result<String, Self::Err>;

  /// Backend name.
  fn name(&self) -> Result<String, Self::Err>;

  /// Backend version.
  fn version(&self) -> Result<String, Self::Err>;

  /// Backend shading language version.
  fn shading_lang_version(&self) -> Result<String, Self::Err>;

  /// Create a new [`VertexArray`].
  fn new_vertex_array(
    &self,
    vertices: &VertexArrayData,
    instances: &VertexArrayData,
    indices: &[u32],
  ) -> Result<VertexArray, Self::Err>;

  /// Update vertices in a [`VertexArray`].
  fn update_vertex_array(
    &self,
    vertex_array: &VertexArray,
    update: VertexArrayUpdate,
  ) -> Result<(), Self::Err>;

  fn new_render_targets(
    &self,
    color_attachments: HashSet<ColorAttachment>,
    depth_stencil_attachment: Option<DepthStencilAttachment>,
  ) -> Result<RenderTargets, Self::Err>;

  /// Create a new [`Shader`].
  fn new_shader(&self, sources: &ShaderSources) -> Result<Shader, Self::Err>;

  /// Create a new [`Uniform`].
  fn new_uniform(&self, shader: &Shader, name: &str, ty: UniformType)
    -> Result<Uniform, Self::Err>;

  /// Create a new [`UniformBuffer`].
  fn new_uniform_buffer(&self, shader: &Shader, name: &str) -> Result<UniformBuffer, Self::Err>;

  /// Set a [`Uniform`].
  fn set_uniform(
    &self,
    shader: &Shader,
    uniform: &Uniform,
    value: *const u8,
  ) -> Result<(), Self::Err>;

  fn new_texture(
    &self,
    storage: texture::Storage,
    sampling: TextureSampling,
  ) -> Result<Texture, Self::Err>;

  fn resize_texture(&self, texture: &Texture, storage: texture::Size) -> Result<(), Self::Err>;

  fn set_texels(
    &self,
    texture: &Texture,
    rect: texture::Rect,
    mipmaps: bool,
    level: usize,
    texels: *const u8,
  ) -> Result<(), Self::Err>;

  fn clear_texels(
    &self,
    texture: &Texture,
    rect: texture::Rect,
    mipmaps: bool,
    clear_value: *const u8,
  ) -> Result<(), Self::Err>;

  fn new_cmd_buf(&self) -> Result<CmdBuf, Self::Err>;

  fn cmd_buf_blending(cmd_buf: &CmdBuf, blending: BlendingMode) -> Result<(), Self::Err>;

  fn cmd_buf_depth_test(cmd_buf: &CmdBuf, depth_test: DepthTest) -> Result<(), Self::Err>;

  fn cmd_buf_depth_write(cmd_buf: &CmdBuf, depth_write: DepthWrite) -> Result<(), Self::Err>;

  fn cmd_buf_stencil_test(cmd_buf: &CmdBuf, stencil_test: StencilTest) -> Result<(), Self::Err>;

  fn cmd_buf_face_culling(cmd_buf: &CmdBuf, face_culling: FaceCulling) -> Result<(), Self::Err>;

  fn cmd_buf_viewport(cmd_buf: &CmdBuf, viewport: Viewport) -> Result<(), Self::Err>;

  fn cmd_buf_scissor(cmd_buf: &CmdBuf, scissor: Scissor) -> Result<(), Self::Err>;

  fn cmd_buf_clear_color(cmd_buf: &CmdBuf, clear_color: Option<RGBA>) -> Result<(), Self::Err>;

  fn cmd_buf_clear_depth(cmd_buf: &CmdBuf, clear_depth: Option<f32>) -> Result<(), Self::Err>;

  fn cmd_buf_srgb(cmd_buf: &CmdBuf, srgb: bool) -> Result<(), Self::Err>;

  fn cmd_buf_bind_render_targets(
    cmd_buf: &CmdBuf,
    render_targets: &RenderTargets,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_bind_shader(cmd_buf: &CmdBuf, shader: &Shader) -> Result<(), Self::Err>;

  fn cmd_buf_draw_vertex_array(
    cmd_buf: &CmdBuf,
    vertex_array: &VertexArray,
  ) -> Result<(), Self::Err>;
}
