use piksels_backend::{vertex_array::VertexArrayInfo, Backend};

use crate::vertex_array::VertexArray;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device<B> {
  backend: B,
}

impl<B> Device<B>
where
  B: Backend,
{
  pub fn new_vertex_array(
    &mut self,
    vertices: VertexArrayInfo,
    instances: VertexArrayInfo,
    indices: Vec<u32>,
  ) -> Result<VertexArray, B::Err> {
    self
      .backend
      .new_vertex_array(vertices, instances, indices)
      .map(|raw| VertexArray { raw })
  }
}
