use std::collections::HashSet;

use piksels_backend::{
  render_targets::{ColorAttachmentPoint, DepthStencilAttachmentPoint},
  shader::ShaderSources,
  texture::{Sampling, Storage},
  vertex_array::VertexArrayData,
  Backend, BackendInfo,
};

use crate::{
  pipeline::CmdBuf, render_targets::RenderTargets, shader::Shader, texture::Texture,
  vertex_array::VertexArray,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device<B> {
  backend: B,
}

impl<B> Device<B>
where
  B: Backend,
{
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
    indices: Vec<u32>,
  ) -> Result<VertexArray<B>, B::Err> {
    self
      .backend
      .new_vertex_array(&vertices, &instances, &indices)
      .map(|raw| VertexArray::from_raw(raw, vertices, instances, indices))
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
}
