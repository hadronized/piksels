//! Cached backend state.
//!
//! Backends implement a procedural version of the API. However, when we issue a call to a backend function twice with
//! the same parameter, oftentimes, we could have ommitted calling the function a second time (e.g. binding twice
//! the same resource). For this reason, backends functions are not directly called, but instead we use a cache to
//! check whether function parameters / state have changed.

use std::collections::HashMap;

use crate::{
  blending::{Equation, Factor},
  color::RGBA32F,
  depth_stencil::{Comparison, DepthWrite, StencilFunc},
  face_culling::{FaceCullingFace, FaceCullingOrder},
  scissor::ScissorRegion,
  viewport::Viewport,
  Backend, BackendInfo, Scarce,
};

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

  // pipeline variables
  viewport: Cached<Viewport>,
  clear_color: Cached<RGBA32F>,
  clear_depth: Cached<f32>,
  clear_stencil: Cached<i32>,
  blending_state: Cached<bool>,
  blending_equations: Cached<[Equation; 2]>,
  blending_factors: Cached<[Factor; 4]>,
  depth_test: Cached<bool>,
  depth_test_comparison: Cached<Comparison>,
  depth_write: Cached<DepthWrite>,
  stencil_test: Cached<bool>,
  stencil_func: Cached<StencilFunc>,
  face_culling: Cached<bool>,
  face_culling_order: Cached<FaceCullingOrder>,
  face_culling_face: Cached<FaceCullingFace>,
  scissor: Cached<bool>,
  scissor_region: Cached<ScissorRegion>,
  primitive_restart: Cached<bool>,
  bound_uniform_buffer: Cached<B::UniformBuffer>,
  bound_render_targets: Cached<B::RenderTargets>,
  bound_shader: Cached<B::Shader>,
  srgb: Cached<bool>,
  author: Cached<String>,
  name: Cached<String>,
  version: Cached<String>,
  shading_lang_version: Cached<String>,
  info: Cached<BackendInfo>,
}

impl<B> Default for Cache<B>
where
  B: Backend,
{
  fn default() -> Self {
    Self {
      vertex_arrays: Default::default(),
      render_targets: Default::default(),
      color_attachments: Default::default(),
      depth_stencil_attachments: Default::default(),
      shaders: Default::default(),
      uniforms: Default::default(),
      uniform_buffers: Default::default(),
      textures: Default::default(),
      cmd_bufs: Default::default(),
      viewport: Default::default(),
      clear_color: Default::default(),
      clear_depth: Default::default(),
      clear_stencil: Default::default(),
      blending_state: Default::default(),
      blending_equations: Default::default(),
      blending_factors: Default::default(),
      depth_test: Default::default(),
      depth_test_comparison: Default::default(),
      depth_write: Default::default(),
      stencil_test: Default::default(),
      stencil_func: Default::default(),
      face_culling: Default::default(),
      face_culling_order: Default::default(),
      face_culling_face: Default::default(),
      scissor: Default::default(),
      scissor_region: Default::default(),
      primitive_restart: Default::default(),
      bound_uniform_buffer: Default::default(),
      bound_render_targets: Default::default(),
      bound_shader: Default::default(),
      srgb: Default::default(),
      author: Default::default(),
      name: Default::default(),
      version: Default::default(),
      shading_lang_version: Default::default(),
      info: Default::default(),
    }
  }
}

macro_rules! cache_methods_scarce_resource {
  ($(track = $track:ident, untrack = $untrack:ident ($map:ident : $ty:ident)),* $(,)?) => {
    $(
      pub fn $track(&mut self, res: &B::$ty) {
        self.$map.insert(res.scarce_index(), res.scarce_clone());
      }

      pub fn $untrack(&mut self, res: &B::$ty) {
        self.$map.remove(&res.scarce_index());
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
    track = track_vertex_array, untrack = untrack_vertex_array (vertex_arrays: VertexArray),
    track = track_render_targets, untrack = untrack_render_targets (render_targets: RenderTargets),
    track = track_color_attachment, untrack = untrack_color_attachment (color_attachments: ColorAttachment),
    track = track_depth_stencil_attachment, untrack = untrack_depth_stencil_attachment (depth_stencil_attachments: DepthStencilAttachment),
    track = track_shader, untrack = untrack_shader (shaders: Shader),
    track = track_uniform, untrack = untrack_uniform (uniforms: Uniform),
    track = track_uniform_buffer, untrack = untrack_uniform_buffer (uniform_buffers: UniformBuffer),
    track = track_texture, untrack = untrack_texture (textures: Texture),
    track = track_cmd_buf, untrack = untrack_cmd_buf (cmd_bufs: CmdBuf),
  );

  cache_methods_pipeline_vars!(
    viewport: Viewport,
    clear_color: RGBA32F,

    clear_depth: f32,
    clear_stencil: i32,
    blending_state: bool,
    blending_equations: [Equation; 2],
    blending_factors: [Factor; 4],
    depth_test: bool,
    depth_test_comparison: Comparison,
    depth_write: DepthWrite,
    stencil_test: bool,
    stencil_func: StencilFunc,
    face_culling: bool,
    face_culling_order: FaceCullingOrder,
    face_culling_face: FaceCullingFace,
    scissor: bool,
    scissor_region: ScissorRegion,
    primitive_restart: bool,
    bound_uniform_buffer: B::UniformBuffer,
    bound_render_targets: B::RenderTargets,
    bound_shader: B::Shader,
    srgb: bool,
    author: String,
    name: String,
    version: String,
    shading_lang_version: String,
    info: BackendInfo,
  );
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
  T: PartialEq,
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
  pub fn set_if_invalid(&mut self, value: T, f: impl FnOnce()) -> bool {
    match self.0 {
      Some(ref x) if x == &value => false,

      _ => {
        self.0 = Some(value);
        f();
        true
      }
    }
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
