use vertex_array::{VertexArray, VertexArrayData, VertexArrayUpdate};

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

  /// Backend author.
  fn author(&self) -> Result<String, Self::Err>;

  /// Backend name.
  fn name(&self) -> Result<String, Self::Err>;

  /// Backend version.
  fn version(&self) -> Result<String, Self::Err>;

  /// Backend shading language version.
  fn shading_lang_version(&self) -> Result<String, Self::Err>;

  /// Create a new [`VertexArray`].
  fn new_vertex_array(
    &mut self,
    vertices: &VertexArrayData,
    instances: &VertexArrayData,
    indices: &[u32],
  ) -> Result<VertexArray, Self::Err>;

  /// Update vertices in a [`VertexArray`].
  fn update_vertex_array(
    &mut self,
    vertex_array: &mut VertexArray,
    update: VertexArrayUpdate,
  ) -> Result<(), Self::Err>;
}
