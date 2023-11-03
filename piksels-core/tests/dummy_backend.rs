use std::fmt::Display;

use piksels_backend::{
  color::RGBA32F,
  error::Error,
  extension::{
    logger::{BackendLogger, LogEntry, LogLevel, Logger, LoggerExt},
    ExtensionsBuilder,
  },
  info,
  scissor::Scissor,
  viewport::Viewport,
  Backend, BackendInfo, Scarce,
};
use piksels_core::device::Device;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum DummyBackendError {
  Common(Error),
  Unimplemented,
}

impl From<Error> for DummyBackendError {
  fn from(e: Error) -> Self {
    DummyBackendError::Common(e)
  }
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DummyResourceBindingPoint;

impl Scarce<DummyBackend> for DummyResourceBindingPoint {
  fn scarce_index(&self) {}

  fn scarce_clone(&self) -> Self {
    DummyResourceBindingPoint
  }
}

impl Display for DummyResourceBindingPoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("DummyResourceBindingPoint")
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DummyShaderBindingPoint;

impl Scarce<DummyBackend> for DummyShaderBindingPoint {
  fn scarce_index(&self) {}

  fn scarce_clone(&self) -> Self {
    DummyShaderBindingPoint
  }
}

impl Display for DummyShaderBindingPoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("DummyShaderBindingPoint")
  }
}

#[derive(Debug)]
struct DummyLogger;

impl Logger for DummyLogger {
  fn log(&self, log_entry: LogEntry) {
    println!(
      "{file}:{line}:{column} [{module}] | {level:?} | {msg}",
      file = log_entry.file,
      line = log_entry.line,
      column = log_entry.column,
      module = log_entry.module,
      level = log_entry.level,
      msg = log_entry.msg,
    );
  }
}

struct DummyBackend {
  logger_level: LogLevel,
  logger: Box<dyn 'static + Logger>,
}

impl BackendLogger for DummyBackend {
  fn log(&self, log_entry: LogEntry) {
    if log_entry.level <= self.logger_level {
      self.logger.log(log_entry)
    }
  }
}

impl Backend for DummyBackend {
  type CmdBuf = DummyResource;
  type ColorAttachment = DummyResource;
  type DepthStencilAttachment = DummyResource;
  type Err = DummyBackendError;
  type RenderTargets = DummyResource;
  type ScarceIndex = ();
  type Shader = DummyResource;
  type ShaderTextureBindingPoint = DummyShaderBindingPoint;
  type ShaderUniformBufferBindingPoint = DummyShaderBindingPoint;
  type SwapChain = DummyResource;
  type Texture = DummyResource;
  type TextureBindingPoint = DummyResourceBindingPoint;
  type Uniform = DummyResource;
  type UniformBuffer = DummyResource;
  type UniformBufferBindingPoint = DummyResourceBindingPoint;
  type VertexArray = DummyResource;

  fn build(
    extensions: ExtensionsBuilder<LoggerExt<impl 'static + Logger>>,
  ) -> Result<Self, Self::Err> {
    Ok(DummyBackend {
      logger_level: extensions.logger.level_filter,
      logger: Box::new(extensions.logger.logger),
    })
  }

  fn author(&self) -> Result<String, Self::Err> {
    info!(self, "getting author");
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

  fn get_uniform_buffer(
    _shader: &Self::Shader,
    _name: &str,
  ) -> Result<Self::UniformBuffer, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn get_texture_binding_point(
    &self,
    _index: usize,
  ) -> Result<Self::TextureBindingPoint, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn get_uniform_buffer_binding_point(
    &self,
    _index: usize,
  ) -> Result<Self::UniformBufferBindingPoint, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn get_shader_texture_binding_point(
    _shader: &Self::Shader,
    _name: &str,
  ) -> Result<Self::ShaderTextureBindingPoint, Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  /// Get a uniform buffer binding point from a shader.
  fn get_shader_uniform_buffer_binding_point(
    _shader: &Self::Shader,
    _name: &str,
  ) -> Result<Self::ShaderUniformBufferBindingPoint, Self::Err> {
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

  fn cmd_buf_viewport(_cmd_buf: &Self::CmdBuf, _viewport: Viewport) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_scissor(_cmd_buf: &Self::CmdBuf, _scissor: Scissor) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_clear_color(_cmd_buf: &Self::CmdBuf, _clear_color: RGBA32F) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_clear_depth(_cmd_buf: &Self::CmdBuf, _clear_depth: f32) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_srgb(_cmd_buf: &Self::CmdBuf, _srgb: bool) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_set_uniform(
    _cmd_buf: &Self::CmdBuf,
    _uniform: &Self::Uniform,
    _value: *const u8,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_bind_texture(
    _cmd_buf: &Self::CmdBuf,
    _texture: &Self::Texture,
    _binding_point: &Self::TextureBindingPoint,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_associate_texture_binding_point(
    _cmd_buf: &Self::CmdBuf,
    _texture_binding_point: &Self::TextureBindingPoint,
    _shader_binding_point: &Self::ShaderTextureBindingPoint,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_bind_uniform_buffer(
    _cmd_buf: &Self::CmdBuf,
    _uniform_buffer: &Self::UniformBuffer,
    _binding_point: &Self::UniformBufferBindingPoint,
  ) -> Result<(), Self::Err> {
    Err(DummyBackendError::Unimplemented)
  }

  fn cmd_buf_associate_uniform_buffer_binding_point(
    _cmd_buf: &Self::CmdBuf,
    _uniform_buffer_binding_point: &Self::UniformBufferBindingPoint,
    _shader_uniform_buffer_binding_point: &Self::ShaderUniformBufferBindingPoint,
  ) -> Result<(), Self::Err> {
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

  fn swap_chain_render_targets(
    _swap_chain: &Self::SwapChain,
  ) -> Result<Self::RenderTargets, Self::Err> {
    Err(DummyBackendError::Unimplemented)
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
  let init = || {
    DummyBackend::build(
      ExtensionsBuilder::default().logger(LoggerExt::new(LogLevel::Trace, DummyLogger)),
    )
  };
  let backend: Result<DummyBackend, DummyBackendError> = init();
  let backend = backend.unwrap();

  let device = Device::new(backend).unwrap();

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
