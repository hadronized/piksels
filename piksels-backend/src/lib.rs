use std::collections::HashSet;

use texture::{Texture, TextureSampling};

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
}
