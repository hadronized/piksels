use vertex_array::{VertexArray, VertexArrayInfo};

pub mod blending;
pub mod depth_stencil;
pub mod face_culling;
pub mod pixel;
pub mod primitive;
pub mod scissor;
pub mod vertex;
pub mod vertex_array;

pub trait Backend {
  type Err;

  fn new_vertex_array(
    &mut self,
    vertices: VertexArrayInfo,
    instances: VertexArrayInfo,
    indices: Vec<u32>,
  ) -> Result<VertexArray, Self::Err>;
}
