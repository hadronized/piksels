//! Cached backend state.
//!
//! Backends implement a procedural version of the API. However, when we issue a call to a backend function twice with
//! the same parameter, oftentimes, we could have not ommitted calling the function a second time (e.g. binding twice
//! the same resource). For this reason, backends function are not directly called, but instead we use a cache to ensure
//! that function parameters / state have changed.

use std::collections::HashMap;

use crate::{
  blending::{Equation, Factor},
  color::RGBA32F,
  depth_stencil::{Comparison, DepthWrite, StencilOp},
  face_culling::{FaceCullingFace, FaceCullingOrder},
  scissor::ScissorRegion,
  viewport::Viewport,
  Backend, BackendInfo, ScarceIndex,
};

#[derive(Debug)]
pub struct Cache<B>
where
  B: Backend,
{
  // scarce resources allocated by this backend
  vertex_arrays: HashMap<ScarceIndex, B::VertexArray>,
  render_targets: HashMap<ScarceIndex, B::RenderTargets>,
  color_attachments: HashMap<ScarceIndex, B::ColorAttachment>,
  depth_stencil_attachments: HashMap<ScarceIndex, B::DepthStencilAttachment>,
  shaders: HashMap<ScarceIndex, B::Shader>,
  uniforms: HashMap<ScarceIndex, B::Uniform>,
  uniform_buffers: HashMap<ScarceIndex, B::UniformBuffer>,
  textures: HashMap<ScarceIndex, B::Texture>,
  cmd_bufs: HashMap<ScarceIndex, B::CmdBuf>,

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
  stencil_func: Cached<(Comparison, u8, u8)>, // TODO: proper typing?
  stencil_ops: Cached<[StencilOp; 3]>,
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
#[derive(Debug, Default)]
pub struct Cached<T>(Option<T>);

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
