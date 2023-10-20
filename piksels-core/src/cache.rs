//! Cached backend state.
//!
//! Backends implement a procedural version of the API. However, when we issue a call to a backend function twice with
//! the same parameter, oftentimes, we could have ommitted calling the function a second time (e.g. binding twice
//! the same resource). For this reason, backends functions are not directly called, but instead we use a cache to
//! check whether function parameters / state have changed.

use std::collections::HashMap;

use piksels_backend::{
  blending::BlendingMode,
  color::RGBA32F,
  depth_stencil::{DepthTest, DepthWrite, StencilTest},
  face_culling::FaceCulling,
  scissor::Scissor,
  viewport::Viewport,
  Backend, BackendInfo, Scarce,
};

use crate::units::Units;

#[derive(Debug)]
pub struct Cache<B>
where
  B: Backend,
{
  // scarce resources allocated by this backend
  vertex_arrays: HashMap<B::ScarceIndex, B::VertexArray>,
  render_targets: HashMap<B::ScarceIndex, B::RenderTargets>,
  color_attachments: HashMap<B::ScarceIndex, B::ColorAttachment>,
  depth_stencil_attachments: HashMap<B::ScarceIndex, B::DepthStencilAttachment>,
  shaders: HashMap<B::ScarceIndex, B::Shader>,
  uniforms: HashMap<B::ScarceIndex, B::Uniform>,
  uniform_buffers: HashMap<B::ScarceIndex, B::UniformBuffer>,
  textures: HashMap<B::ScarceIndex, B::Texture>,
  cmd_bufs: HashMap<B::ScarceIndex, B::CmdBuf>,
  swap_chains: HashMap<B::ScarceIndex, B::SwapChain>,

  // pipeline variables
  viewport: Cached<Viewport>,
  clear_color: Cached<RGBA32F>,
  clear_depth: Cached<f32>,
  clear_stencil: Cached<i32>,
  blending: Cached<BlendingMode>,
  depth_test: Cached<DepthTest>,
  depth_write: Cached<DepthWrite>,
  stencil_test: Cached<StencilTest>,
  face_culling: Cached<FaceCulling>,
  srgb: Cached<bool>,
  scissor: Cached<Scissor>,
  primitive_restart: Cached<bool>,
  // texture support
  texture_units: Units<B, B::TextureUnit>,
  bound_textures: HashMap<B::ScarceIndex, B::TextureUnit>,
  // uniform buffer support
  uniform_buffer_units: Units<B, B::UniformBufferUnit>,
  bound_uniform_buffers: HashMap<B::ScarceIndex, B::UniformBufferUnit>,
  // pipeline resources (render targets, shaders)
  bound_render_targets: Cached<B::RenderTargets>,
  bound_shader: Cached<B::Shader>,
  // query info; not properly “cached” — instead they are more likely either never queried, or queried once and kept
  // around for ever
  author: Option<String>,
  name: Option<String>,
  version: Option<String>,
  shading_lang_version: Option<String>,
  info: Option<BackendInfo>,
}

impl<B> Drop for Cache<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    for vertex_array in self.vertex_arrays.values() {
      B::drop_vertex_array(vertex_array);
    }

    for render_targets in self.render_targets.values() {
      B::drop_render_targets(render_targets);
    }

    for shader in self.shaders.values() {
      B::drop_shader(shader);
    }

    for texture in self.textures.values() {
      B::drop_texture(texture);
    }

    for cmd_buf in self.cmd_bufs.values() {
      B::drop_cmd_buf(cmd_buf);
    }

    for swap_chain in self.swap_chains.values() {
      B::drop_swap_chain(swap_chain);
    }
  }
}

macro_rules! cache_methods_scarce_resource {
  ($(track = $track:ident, untrack = $untrack:ident $(, drop = $drop:ident)? ($map:ident : $ty:ident)),* $(,)?) => {
    $(
      pub fn $track(&mut self, res: &B::$ty) {
        self.$map.insert(res.scarce_index(), res.scarce_clone());
      }

      pub fn $untrack(&mut self, res: &B::$ty) {
        self.$map.remove(&res.scarce_index());
        $(B::$drop(res);)?
      }
    )*
  };
}

macro_rules! cache_methods_pipeline_vars {
  ($($name:ident: $ty:ty),* $(,)?) => {
    $(
      pub fn $name(&mut self) -> &mut Cached<$ty> {
        &mut self.$name
      }
    )*
  }
}

impl<B> Cache<B>
where
  B: Backend,
{
  cache_methods_scarce_resource!(
    track = track_vertex_array, untrack = untrack_vertex_array, drop = drop_vertex_array (vertex_arrays: VertexArray),
    track = track_render_targets, untrack = untrack_render_targets, drop = drop_render_targets (render_targets: RenderTargets),
    track = track_color_attachment, untrack = untrack_color_attachment (color_attachments: ColorAttachment),
    track = track_depth_stencil_attachment, untrack = untrack_depth_stencil_attachment (depth_stencil_attachments: DepthStencilAttachment),
    track = track_shader, untrack = untrack_shader, drop = drop_shader (shaders: Shader),
    track = track_uniform, untrack = untrack_uniform (uniforms: Uniform),
    track = track_uniform_buffer, untrack = untrack_uniform_buffer (uniform_buffers: UniformBuffer),
    track = track_texture, untrack = untrack_texture, drop = drop_texture (textures: Texture),
    track = track_cmd_buf, untrack = untrack_cmd_buf, drop = drop_cmd_buf (cmd_bufs: CmdBuf),
    track = track_swap_chain, untrack = untrack_swap_chain, drop = drop_swap_chain (swap_chains: SwapChain),
  );

  cache_methods_pipeline_vars!(
    viewport: Viewport,
    clear_color: RGBA32F,

    clear_depth: f32,
    clear_stencil: i32,
    blending: BlendingMode,
    depth_test: DepthTest,
    depth_write: DepthWrite,
    stencil_test: StencilTest,
    face_culling: FaceCulling,
    srgb: bool,
    scissor: Scissor,
    primitive_restart: bool,
    bound_render_targets: B::RenderTargets,
    bound_shader: B::Shader,
  );

  pub fn new(backend: &B) -> Result<Self, B::Err> {
    Ok(Self {
      vertex_arrays: Default::default(),
      render_targets: Default::default(),
      color_attachments: Default::default(),
      depth_stencil_attachments: Default::default(),
      shaders: Default::default(),
      uniforms: Default::default(),
      uniform_buffers: Default::default(),
      textures: Default::default(),
      cmd_bufs: Default::default(),
      swap_chains: Default::default(),
      viewport: Default::default(),
      clear_color: Default::default(),
      clear_depth: Default::default(),
      clear_stencil: Default::default(),
      blending: Default::default(),
      depth_test: Default::default(),
      depth_write: Default::default(),
      stencil_test: Default::default(),
      face_culling: Default::default(),
      srgb: Default::default(),
      scissor: Default::default(),
      primitive_restart: Default::default(),
      texture_units: Units::new(backend.max_texture_units()?),
      bound_textures: HashMap::default(),
      uniform_buffer_units: Units::new(backend.max_uniform_buffer_units()?),
      bound_uniform_buffers: HashMap::default(),
      bound_render_targets: Default::default(),
      bound_shader: Default::default(),
      author: Default::default(),
      name: Default::default(),
      version: Default::default(),
      shading_lang_version: Default::default(),
      info: Default::default(),
    })
  }

  pub fn author(&mut self) -> &mut Option<String> {
    &mut self.author
  }

  pub fn name(&mut self) -> &mut Option<String> {
    &mut self.name
  }

  pub fn version(&mut self) -> &mut Option<String> {
    &mut self.version
  }

  pub fn shading_lang_version(&mut self) -> &mut Option<String> {
    &mut self.shading_lang_version
  }

  pub fn info(&mut self) -> &mut Option<BackendInfo> {
    &mut self.info
  }
}

impl<B> Cache<B> where B: Backend {}

/// Cached value.
///
/// A cached value is used to prevent issuing costy GPU commands if we know the target value is
/// already set to what the command tries to set. For instance, if you ask to use texture ID
/// `34` once, that value will be set on the GPU and cached on our side. Later, if no other texture
/// setting has occurred, if you ask to use the texture ID `34` again, because the value is cached,
/// we know the GPU is already using it, so we don’t have to perform anything GPU-wise.
///
/// This optimization has limits and sometimes, because of side-effects, it is not possible to cache
/// something correctly.
#[derive(Debug)]
pub struct Cached<T>(Option<T>);

impl<T> Default for Cached<T> {
  fn default() -> Self {
    Cached(None)
  }
}

impl<T> Cached<T>
where
  T: PartialEq + Clone,
{
  /// Explicitly invalidate a value.
  pub fn invalidate(&mut self) {
    self.0 = None;
  }

  /// Explicitly set a value.
  pub fn set(&mut self, value: T) -> Option<T> {
    self.0.replace(value)
  }

  /// Set the value if invalid, then call the function.
  ///
  /// If the value was still valid, returns `true`.
  ///
  /// See more: [`Cached::is_invalid`].
  pub fn set_if_invalid<E>(
    &mut self,
    value: &T,
    f: impl FnOnce() -> Result<(), E>,
  ) -> Result<bool, E> {
    match self.0 {
      Some(ref x) if x == value => Ok(false),

      _ => {
        self.0 = Some(value.clone());
        f().map(|_| true)
      }
    }
  }

  /// Check whether a value is cached, whatever it is.
  pub fn exists(&self) -> bool {
    self.0.is_some()
  }

  /// Check whether the cached value is invalid regarding a value.
  ///
  /// A value is invalid if it was never set, or if it’s different from the parameter one.
  pub fn is_invalid(&self, new_val: &T) -> bool {
    match &self.0 {
      Some(ref t) => t != new_val,
      _ => true,
    }
  }
}
