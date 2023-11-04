use std::collections::HashSet;

use piksels_backend::{
  render_targets::{ColorAttachmentPoint, DepthStencilAttachmentPoint},
  shader::ShaderSources,
  swap_chain::SwapChainMode,
  texture::{Sampling, Storage},
  vertex_array::VertexArrayData,
  Backend, BackendInfo,
};

use crate::{
  cmd_buf::CmdBuf,
  render_targets::RenderTargets,
  shader::{Shader, UniformBufferBindingPoint},
  swap_chain::SwapChain,
  texture::{Texture, TextureBindingPoint},
  vertex_array::VertexArray,
};

#[derive(Debug)]
pub struct Device<B>
where
  B: Backend,
{
  backend: B,
}

impl<B> Device<B>
where
  B: Backend,
{
  pub fn new(backend: B) -> Result<Self, B::Err> {
    Ok(Self { backend })
  }

  pub fn author(&self) -> Result<String, B::Err> {
    self.backend.author()
  }

  pub fn name(&self) -> Result<String, B::Err> {
    self.backend.name()
  }

  pub fn version(&self) -> Result<String, B::Err> {
    self.backend.version()
  }

  pub fn shading_lang_version(&self) -> Result<String, B::Err> {
    self.backend.shading_lang_version()
  }

  pub fn info(&self) -> Result<BackendInfo, B::Err> {
    self.backend.info()
  }

  pub fn new_vertex_array(
    &self,
    vertices: VertexArrayData,
    instances: VertexArrayData,
    indices: impl Into<Vec<u32>>,
  ) -> Result<VertexArray<B>, B::Err> {
    let indices = indices.into();
    let vertex_count = if indices.is_empty() {
      vertices.len()
    } else {
      indices.len()
    };

    self
      .backend
      .new_vertex_array(&vertices, &instances, &indices)
      .map(|raw| VertexArray::from_raw(raw, vertex_count))
  }

  pub fn new_render_targets(
    &self,
    color_attachment_points: HashSet<ColorAttachmentPoint>,
    depth_stencil_attachment_point: Option<DepthStencilAttachmentPoint>,
    storage: Storage,
  ) -> Result<RenderTargets<B>, B::Err> {
    self
      .backend
      .new_render_targets(
        color_attachment_points,
        depth_stencil_attachment_point,
        storage,
      )
      .map(RenderTargets::from_raw)
  }

  pub fn new_shader(&self, sources: ShaderSources) -> Result<Shader<B>, B::Err> {
    self.backend.new_shader(sources).map(Shader::from_raw)
  }

  pub fn new_texture(&self, storage: Storage, sampling: Sampling) -> Result<Texture<B>, B::Err> {
    self
      .backend
      .new_texture(storage, sampling)
      .map(Texture::from_raw)
  }

  pub fn new_cmd_buf(&self) -> Result<CmdBuf<B>, B::Err> {
    self.backend.new_cmd_buf().map(CmdBuf::from_raw)
  }

  pub fn new_swap_chain(
    &self,
    width: u32,
    height: u32,
    mode: SwapChainMode,
  ) -> Result<SwapChain<B>, B::Err> {
    self
      .backend
      .new_swap_chain(width, height, mode)
      .map(SwapChain::from_raw)
  }

  pub fn get_texture_binding_point(&self, index: usize) -> Result<TextureBindingPoint<B>, B::Err> {
    self
      .backend
      .get_texture_binding_point(index)
      .map(TextureBindingPoint::from_raw)
  }

  pub fn get_uniform_buffer_binding_point(
    &self,
    index: usize,
  ) -> Result<UniformBufferBindingPoint<B>, B::Err> {
    self
      .backend
      .get_uniform_buffer_binding_point(index)
      .map(UniformBufferBindingPoint::from_raw)
  }
}
