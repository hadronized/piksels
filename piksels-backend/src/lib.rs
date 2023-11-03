use std::{collections::HashSet, fmt::Debug, hash::Hash};

use blending::BlendingMode;
use color::RGBA32F;
use depth_stencil::{DepthTest, DepthWrite, StencilTest};
use error::Error;
use extension::{
  logger::{Logger, LoggerExt},
  ExtensionsBuilder,
};
use face_culling::FaceCulling;
use render_targets::{ColorAttachmentPoint, DepthStencilAttachmentPoint};
use scissor::Scissor;
use swap_chain::SwapChainMode;
use texture::{Sampling, Storage};
use viewport::Viewport;

use crate::{
  shader::{ShaderSources, UniformType},
  vertex_array::{VertexArrayData, VertexArrayUpdate},
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
pub mod cache;
pub mod color;
pub mod depth_stencil;
pub mod error;
pub mod extension;
pub mod face_culling;
pub mod pixel;
pub mod primitive;
pub mod render_targets;
pub mod scissor;
pub mod shader;
pub mod swap_chain;
pub mod texture;
pub mod vertex;
pub mod vertex_array;
pub mod viewport;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BackendInfo {
  pub version: &'static str,
  pub git_commit_hash: &'static str,
}

pub trait Scarce<B>: Debug
where
  B: Backend + ?Sized,
{
  fn scarce_index(&self) -> B::ScarceIndex;
  fn scarce_clone(&self) -> Self;
}

pub trait Backend: Sized {
  type Err: From<Error>;

  type CmdBuf: Scarce<Self>;
  type ColorAttachment: Scarce<Self>;
  type DepthStencilAttachment: Scarce<Self>;
  type RenderTargets: Scarce<Self>;
  type ScarceIndex: Clone + Debug + Eq + Hash + Ord + PartialEq + PartialOrd;
  type Shader: Scarce<Self>;
  type ShaderTextureBindingPoint: Scarce<Self>;
  type ShaderUniformBufferBindingPoint: Scarce<Self>;
  type SwapChain: Scarce<Self>;
  type Texture: Scarce<Self>;
  type TextureBindingPoint: Scarce<Self>;
  type Uniform: Scarce<Self>;
  type UniformBuffer: Scarce<Self>;
  type UniformBufferBindingPoint: Scarce<Self>;
  type VertexArray: Scarce<Self>;

  /// Initialize the backend from extensions.
  fn build(
    extensions: ExtensionsBuilder<LoggerExt<impl 'static + Logger>>,
  ) -> Result<Self, Self::Err>;

  /// Backend author.
  fn author(&self) -> Result<String, Self::Err>;

  /// Backend name.
  fn name(&self) -> Result<String, Self::Err>;

  /// Backend version.
  fn version(&self) -> Result<String, Self::Err>;

  /// Backend shading language version.
  fn shading_lang_version(&self) -> Result<String, Self::Err>;

  /// More information about the backend (git hash, etc.).
  fn info(&self) -> Result<BackendInfo, Self::Err>;

  /// Create a new [`VertexArray`].
  fn new_vertex_array(
    &self,
    vertices: &VertexArrayData,
    instances: &VertexArrayData,
    indices: &[u32],
  ) -> Result<Self::VertexArray, Self::Err>;

  /// Drop a [`VertexArray`].
  fn drop_vertex_array(vertex_array: &Self::VertexArray);

  /// Update vertices in a [`VertexArray`].
  fn update_vertex_array(
    vertex_array: &Self::VertexArray,
    update: VertexArrayUpdate,
  ) -> Result<(), Self::Err>;

  fn new_render_targets(
    &self,
    color_attachment_points: HashSet<ColorAttachmentPoint>,
    depth_stencil_attachment_point: Option<DepthStencilAttachmentPoint>,
    storage: Storage,
  ) -> Result<Self::RenderTargets, Self::Err>;

  /// Drop a [`RenderTargets`].
  fn drop_render_targets(render_targets: &Self::RenderTargets);

  /// Obtain the indexed color attachment.
  fn get_color_attachment(
    render_targets: &Self::RenderTargets,
    index: usize,
  ) -> Result<Self::ColorAttachment, Self::Err>;

  /// Obtain the indexed depth/stencil attachment.
  fn get_depth_stencil_attachment(
    render_targets: &Self::RenderTargets,
    index: usize,
  ) -> Result<Self::DepthStencilAttachment, Self::Err>;

  /// Create a new [`Shader`].
  fn new_shader(&self, sources: ShaderSources) -> Result<Self::Shader, Self::Err>;

  /// Drop a [`Shader`].
  fn drop_shader(shader: &Self::Shader);

  /// Create a new [`Uniform`].
  fn get_uniform(
    shader: &Self::Shader,
    name: &str,
    ty: UniformType,
  ) -> Result<Self::Uniform, Self::Err>;

  /// Create a new [`UniformBuffer`].
  fn get_uniform_buffer(
    shader: &Self::Shader,
    name: &str,
  ) -> Result<Self::UniformBuffer, Self::Err>;

  /// Get a texture binding point.
  fn get_texture_binding_point(&self, index: usize)
    -> Result<Self::TextureBindingPoint, Self::Err>;

  /// Get a uniform buffer binding point.
  fn get_uniform_buffer_binding_point(
    &self,
    index: usize,
  ) -> Result<Self::UniformBufferBindingPoint, Self::Err>;

  /// Get a shader texture binding point from a shader.
  fn get_shader_texture_binding_point(
    shader: &Self::Shader,
    name: &str,
  ) -> Result<Self::ShaderTextureBindingPoint, Self::Err>;

  /// Get a uniform buffer binding point from a shader.
  fn get_shader_uniform_buffer_binding_point(
    shader: &Self::Shader,
    name: &str,
  ) -> Result<Self::ShaderUniformBufferBindingPoint, Self::Err>;

  fn new_texture(&self, storage: Storage, sampling: Sampling) -> Result<Self::Texture, Self::Err>;

  fn drop_texture(texture: &Self::Texture);

  fn resize_texture(texture: &Self::Texture, size: texture::Size) -> Result<(), Self::Err>;

  fn set_texels(
    texture: &Self::Texture,
    rect: texture::Rect,
    mipmaps: bool,
    level: usize,
    texels: *const u8,
  ) -> Result<(), Self::Err>;

  fn clear_texels(
    texture: &Self::Texture,
    rect: texture::Rect,
    mipmaps: bool,
    value: *const u8,
  ) -> Result<(), Self::Err>;

  fn new_cmd_buf(&self) -> Result<Self::CmdBuf, Self::Err>;

  fn drop_cmd_buf(cmd_buf: &Self::CmdBuf);

  fn cmd_buf_blending(cmd_buf: &Self::CmdBuf, blending: BlendingMode) -> Result<(), Self::Err>;

  fn cmd_buf_depth_test(cmd_buf: &Self::CmdBuf, depth_test: DepthTest) -> Result<(), Self::Err>;

  fn cmd_buf_depth_write(cmd_buf: &Self::CmdBuf, depth_write: DepthWrite) -> Result<(), Self::Err>;

  fn cmd_buf_stencil_test(
    cmd_buf: &Self::CmdBuf,
    stencil_test: StencilTest,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_face_culling(
    cmd_buf: &Self::CmdBuf,
    face_culling: FaceCulling,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_viewport(cmd_buf: &Self::CmdBuf, viewport: Viewport) -> Result<(), Self::Err>;

  fn cmd_buf_scissor(cmd_buf: &Self::CmdBuf, scissor: Scissor) -> Result<(), Self::Err>;

  fn cmd_buf_clear_color(cmd_buf: &Self::CmdBuf, clear_color: RGBA32F) -> Result<(), Self::Err>;

  fn cmd_buf_clear_depth(cmd_buf: &Self::CmdBuf, clear_depth: f32) -> Result<(), Self::Err>;

  fn cmd_buf_srgb(cmd_buf: &Self::CmdBuf, srgb: bool) -> Result<(), Self::Err>;

  fn cmd_buf_set_uniform(
    cmd_buf: &Self::CmdBuf,
    uniform: &Self::Uniform,
    value: *const u8, // TODO: type with UniformValue trait
  ) -> Result<(), Self::Err>;

  /// Bind a texture.
  fn cmd_buf_bind_texture(
    cmd_buf: &Self::CmdBuf,
    texture: &Self::Texture,
    binding_point: &Self::TextureBindingPoint,
  ) -> Result<(), Self::Err>;

  /// Associate a texture binding point to a shader texture binding point.
  fn cmd_buf_associate_texture_binding_point(
    cmd_buf: &Self::CmdBuf,
    texture_binding_point: &Self::TextureBindingPoint,
    shader_binding_point: &Self::ShaderTextureBindingPoint,
  ) -> Result<(), Self::Err>;

  /// Bind a uniform buffer.
  fn cmd_buf_bind_uniform_buffer(
    cmd_buf: &Self::CmdBuf,
    uniform_buffer: &Self::UniformBuffer,
    binding_point: &Self::UniformBufferBindingPoint,
  ) -> Result<(), Self::Err>;

  /// Associate a uniform buffer binding point to a shader uniform buffer binding point.
  fn cmd_buf_associate_uniform_buffer_binding_point(
    cmd_buf: &Self::CmdBuf,
    uniform_buffer_binding_point: &Self::UniformBufferBindingPoint,
    shader_uniform_buffer_binding_point: &Self::ShaderUniformBufferBindingPoint,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_bind_render_targets(
    cmd_buf: &Self::CmdBuf,
    render_targets: &Self::RenderTargets,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_bind_shader(cmd_buf: &Self::CmdBuf, shader: &Self::Shader) -> Result<(), Self::Err>;

  fn cmd_buf_draw_vertex_array(
    cmd_buf: &Self::CmdBuf,
    vertex_array: &Self::VertexArray,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_finish(cmd_buf: &Self::CmdBuf) -> Result<(), Self::Err>;

  fn new_swap_chain(
    &self,
    width: u32,
    height: u32,
    mode: SwapChainMode,
  ) -> Result<Self::SwapChain, Self::Err>;

  fn drop_swap_chain(swap_chain: &Self::SwapChain);

  fn swap_chain_render_targets(
    swap_chain: &Self::SwapChain,
  ) -> Result<Self::RenderTargets, Self::Err>;

  fn present_render_targets(
    swap_chain: &Self::SwapChain,
    render_targets: &Self::RenderTargets,
  ) -> Result<(), Self::Err>;
}
