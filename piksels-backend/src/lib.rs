use std::collections::HashSet;

use texture::Texture;

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
pub mod depth_stencil;
pub mod face_culling;
pub mod pixel;
pub mod primitive;
pub mod render_targets;
pub mod scissor;
pub mod shader;
pub mod texture;
pub mod vertex;
pub mod vertex_array;
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
    &mut self,
    vertices: &VertexArrayData,
    instances: &VertexArrayData,
    indices: &[u32],
  ) -> Result<VertexArray, Self::Err>;

  /// Update vertices in a [`VertexArray`].
  fn update_vertex_array(
    &mut self,
    vertex_array: &mut VertexArray,
    update: VertexArrayUpdate,
  ) -> Result<(), Self::Err>;

  fn new_render_targets(
    &mut self,
    color_attachments: HashSet<ColorAttachment>,
    depth_stencil_attachment: Option<DepthStencilAttachment>,
  ) -> Result<RenderTargets, Self::Err>;

  /// Create a new [`Shader`].
  fn new_shader(&mut self, sources: &ShaderSources) -> Result<Shader, Self::Err>;

  /// Create a new [`Uniform`].
  fn new_shader_uniform(
    &mut self,
    shader: &Shader,
    name: &str,
    ty: UniformType,
  ) -> Result<Uniform, Self::Err>;

  fn new_shader_uniform_buffer(
    &mut self,
    shader: &Shader,
    name: &str,
  ) -> Result<UniformBuffer, Self::Err>;

  /// Set a [`Uniform`].
  fn set_uniform(
    &mut self,
    shader: &Shader,
    uniform: &Uniform,
    value: *const u8,
  ) -> Result<(), Self::Err>;

  fn new_texture(&mut self, storage: texture::Storage) -> Result<Texture, Self::Err>;
}
