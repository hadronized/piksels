use std::collections::HashSet;

use piksels_backend::{
  render_targets::{ColorAttachmentPoint, DepthStencilAttachmentPoint},
  shader::ShaderSources,
  texture::{Sampling, Storage},
  vertex_array::VertexArrayData,
  Backend,
};

use crate::{
  render_targets::RenderTargets, shader::Shader, texture::Texture, vertex_array::VertexArray,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device<B> {
  backend: B,
}

impl<B> Device<B>
where
  B: Backend,
{
  pub fn backend_author(&self) -> Result<String, B::Err> {
    self.backend.author()
  }

  pub fn backend_name(&self) -> Result<String, B::Err> {
    self.backend.name()
  }

  pub fn backend_version(&self) -> Result<String, B::Err> {
    self.backend.version()
  }

  pub fn backend_shading_lang_version(&self) -> Result<String, B::Err> {
    self.backend.shading_lang_version()
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
}
