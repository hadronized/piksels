#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArray {
  pub(crate) raw: piksels_backend::vertex_array::VertexArray,
}
