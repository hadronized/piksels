use std::{
  collections::HashSet,
  fmt::{Debug, Display},
  hash::Hash,
};

use blending::BlendingMode;
use color::RGBA;
use depth_stencil::{DepthTest, DepthWrite, StencilTest};
use error::Error;
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

pub trait Unit:
  Clone + Debug + Display + Default + Eq + Hash + Ord + PartialEq + PartialOrd
{
  fn next_unit(&self) -> Self;
}

pub trait Backend {
  type Err: From<Error>;

  type CmdBuf: Scarce<Self>;
  type ColorAttachment: Scarce<Self>;
  type DepthStencilAttachment: Scarce<Self>;
  type RenderTargets: Scarce<Self>;
  type ScarceIndex: Clone + Debug + Eq + Hash + Ord + PartialEq + PartialOrd;
  type Shader: Scarce<Self>;
  type SwapChain: Scarce<Self>;
  type Texture: Scarce<Self>;
  type Uniform: Scarce<Self>;
  type UniformBuffer: Scarce<Self>;
  type Unit: Unit;
  type VertexArray: Scarce<Self>;

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

  /// Drop a [`Uniform`].
  fn drop_uniform(uniform: &Self::Uniform);

  /// Create a new [`UniformBuffer`].
  fn get_uniform_buffer(
    shader: &Self::Shader,
    name: &str,
  ) -> Result<Self::UniformBuffer, Self::Err>;

  /// Drop a [`UniformBuffer`].
  fn drop_uniform_buffer(uniform_buffer: &Self::UniformBuffer);

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

  fn cmd_buf_clear_color(
    cmd_buf: &Self::CmdBuf,
    clear_color: Option<RGBA>,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_clear_depth(cmd_buf: &Self::CmdBuf, clear_depth: Option<f32>)
    -> Result<(), Self::Err>;

  fn cmd_buf_srgb(cmd_buf: &Self::CmdBuf, srgb: bool) -> Result<(), Self::Err>;

  fn cmd_buf_set_uniform(
    cmd_buf: &Self::CmdBuf,
    uniform: &Self::Uniform,
    value: *const u8,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_bind_texture(
    cmd_buf: &Self::CmdBuf,
    texture: &Self::Texture,
    unit: &Self::Unit,
  ) -> Result<(), Self::Err>;

  fn cmd_buf_bind_uniform_buffer(
    cmd_buf: &Self::CmdBuf,
    uniform_buffer: &Self::UniformBuffer,
    unit: &Self::Unit,
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

  fn present_render_targets(
    swap_chain: &Self::SwapChain,
    render_targets: &Self::RenderTargets,
  ) -> Result<(), Self::Err>;

  fn max_texture_units(&self) -> Result<Self::Unit, Self::Err>;

  fn max_uniform_buffer_units(&self) -> Result<Self::Unit, Self::Err>;
}
