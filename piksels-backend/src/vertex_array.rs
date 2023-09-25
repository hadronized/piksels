use crate::vertex::VertexAttr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArray {
  handle: usize,
}

impl VertexArray {
  pub fn handle(&self) -> usize {
    self.handle
  }
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
