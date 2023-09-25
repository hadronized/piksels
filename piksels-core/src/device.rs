use piksels_backend::{vertex_array::VertexArrayData, Backend};

use crate::vertex_array::VertexArray;

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
    &mut self,
    vertices: VertexArrayData,
    instances: VertexArrayData,
    indices: Vec<u32>,
  ) -> Result<VertexArray, B::Err> {
    self
      .backend
      .new_vertex_array(&vertices, &instances, &indices)
      .map(|raw| VertexArray::new(raw, vertices, instances, indices))
  }
}
