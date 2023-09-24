use crate::vertex::VertexAttr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArray {
  pub handle: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArrayInfo {
  attrs: Vec<VertexAttr>,
  data: Vec<u8>,
  layout: MemoryLayout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryLayout {
  Interleaved,
  Deinterleaved,
}
