use std::fmt::Display;

use piksels_backend::{Backend, BackendInfo, Scarce};
use piksels_core::device::Device;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum DummyBackendError {
  Unimplemented,
}

impl Display for DummyBackendError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("unimplemented")
  }
}

#[derive(Debug)]
struct DummyResource;

impl Scarce<DummyBackend> for DummyResource {
  fn scarce_index(&self) {}

  fn scarce_clone(&self) -> Self {
    DummyResource
  }
}

#[derive(Debug)]
struct DummyBackend;

impl Backend for DummyBackend {
  type CmdBuf = DummyResource;
  type ColorAttachment = DummyResource;
  type DepthStencilAttachment = DummyResource;
  type Err = DummyBackendError;
  type RenderTargets = DummyResource;
  type ScarceIndex = ();
  type Shader = DummyResource;
  type SwapChain = DummyResource;
  type Texture = DummyResource;
  type Uniform = DummyResource;
  type UniformBuffer = DummyResource;
  type VertexArray = DummyResource;

  fn author(&self) -> Result<String, Self::Err> {
    Ok("Dimitri 'phaazon' Sabadie <dimitri.sabadie@gmail.com>".to_owned())
  }

  fn name(&self) -> Result<String, Self::Err> {
    Ok("DummyBackend".to_owned())
  }

  fn version(&self) -> Result<String, Self::Err> {
    Ok("v1.0.0-super-dummy".to_owned())
  }

  fn shading_lang_version(&self) -> Result<String, Self::Err> {
    Ok("v1.0.0".to_owned())
  }

  fn info(&self) -> Result<BackendInfo, Self::Err> {
    Ok(BackendInfo {
      version: env!("CARGO_PKG_VERSION"),
      git_commit_hash: "HEAD",
    })
  }

  fn new_vertex_array(
    &self,
    _vertices: &piksels_backend::vertex_array::VertexArrayData,
    _instances: &piksels_backend::vertex_array::VertexArrayData,
    _indices: &[u32],
  ) -> Result<Self::VertexArray, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_vertex_array(_vertex_array: &Self::VertexArray) {
    unimplemented!()
  }

  fn update_vertex_array(
    _vertex_array: &Self::VertexArray,
    _update: piksels_backend::vertex_array::VertexArrayUpdate,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn new_render_targets(
    &self,
    _color_attachment_points: std::collections::HashSet<
      piksels_backend::render_targets::ColorAttachmentPoint,
    >,
    _depth_stencil_attachment_point: Option<
      piksels_backend::render_targets::DepthStencilAttachmentPoint,
    >,
    _storage: piksels_backend::texture::Storage,
  ) -> Result<Self::RenderTargets, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_render_targets(_render_targets: &Self::RenderTargets) {
    unimplemented!()
  }

  fn get_color_attachment(
    _render_targets: &Self::RenderTargets,
    _index: usize,
  ) -> Result<Self::ColorAttachment, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn get_depth_stencil_attachment(
    _render_targets: &Self::RenderTargets,
    _index: usize,
  ) -> Result<Self::DepthStencilAttachment, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn new_shader(
    &self,
    _sources: piksels_backend::shader::ShaderSources,
  ) -> Result<Self::Shader, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_shader(_shader: &Self::Shader) {
    unimplemented!()
  }

  fn get_uniform(
    _shader: &Self::Shader,
    _name: &str,
    _ty: piksels_backend::shader::UniformType,
  ) -> Result<Self::Uniform, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_uniform(_uniform: &Self::Uniform) {
    unimplemented!()
  }

  fn get_uniform_buffer(
    _nhader: &Self::Shader,
    _same: &str,
  ) -> Result<Self::UniformBuffer, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_uniform_buffer(_uniform_buffer: &Self::UniformBuffer) {
    unimplemented!()
  }

  fn set_uniform(_uniform: &Self::Uniform, _value: *const u8) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn new_texture(
    &self,
    _storage: piksels_backend::texture::Storage,
    _sampling: piksels_backend::texture::Sampling,
  ) -> Result<Self::Texture, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_texture(_texture: &Self::Texture) {
    unimplemented!()
  }

  fn resize_texture(
    _texture: &Self::Texture,
    _size: piksels_backend::texture::Size,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn set_texels(
    _texture: &Self::Texture,
    _rect: piksels_backend::texture::Rect,
    _mipmaps: bool,
    _level: usize,
    _texels: *const u8,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn clear_texels(
    _texture: &Self::Texture,
    _rect: piksels_backend::texture::Rect,
    _mipmaps: bool,
    _value: *const u8,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn new_cmd_buf(&self) -> Result<Self::CmdBuf, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_cmd_buf(_cmd_buf: &Self::CmdBuf) {
    unimplemented!()
  }

  fn cmd_buf_blending(
    _cmd_buf: &Self::CmdBuf,
    _blending: piksels_backend::blending::BlendingMode,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_depth_test(
    _cmd_buf: &Self::CmdBuf,
    _depth_test: piksels_backend::depth_stencil::DepthTest,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_depth_write(
    _cmd_buf: &Self::CmdBuf,
    _depth_write: piksels_backend::depth_stencil::DepthWrite,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_stencil_test(
    _cmd_buf: &Self::CmdBuf,
    _stencil_test: piksels_backend::depth_stencil::StencilTest,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_face_culling(
    _cmd_buf: &Self::CmdBuf,
    _face_culling: piksels_backend::face_culling::FaceCulling,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_viewport(
    _cmd_buf: &Self::CmdBuf,
    _viewport: piksels_backend::viewport::Viewport,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_scissor(
    _cmd_buf: &Self::CmdBuf,
    _scissor: piksels_backend::scissor::Scissor,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_clear_color(
    _cmd_buf: &Self::CmdBuf,
    _clear_color: Option<piksels_backend::color::RGBA>,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_clear_depth(
    _cmd_buf: &Self::CmdBuf,
    _clear_depth: Option<f32>,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_srgb(_cmd_buf: &Self::CmdBuf, _srgb: bool) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_bind_render_targets(
    _cmd_buf: &Self::CmdBuf,
    _render_targets: &Self::RenderTargets,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_bind_shader(_cmd_buf: &Self::CmdBuf, _shader: &Self::Shader) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_draw_vertex_array(
    _cmd_buf: &Self::CmdBuf,
    _vertex_array: &Self::VertexArray,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_finish(_cmd_buf: &Self::CmdBuf) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn new_swap_chain(
    &self,
    _width: u32,
    _height: u32,
    _mode: piksels_backend::swap_chain::SwapChainMode,
  ) -> Result<Self::SwapChain, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn drop_swap_chain(_swap_chain: &Self::SwapChain) {
    unimplemented!()
  }

  fn present_render_targets(
    _swap_chain: &Self::SwapChain,
    _render_targets: &Self::RenderTargets,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }
}

#[test]
fn dummy_backend_info() {
  let device = Device::new(DummyBackend);

  assert_eq!(
    device.author(),
    Ok("Dimitri 'phaazon' Sabadie <dimitri.sabadie@gmail.com>".to_owned())
  );
  assert_eq!(device.name(), Ok("DummyBackend".to_owned()));
  assert_eq!(device.version(), Ok("v1.0.0-super-dummy".to_owned()));
  assert_eq!(device.shading_lang_version(), Ok("v1.0.0".to_owned()));
  assert_eq!(
    device.info(),
    Ok(BackendInfo {
      version: env!("CARGO_PKG_VERSION"),
      git_commit_hash: "HEAD"
    })
  );
}
