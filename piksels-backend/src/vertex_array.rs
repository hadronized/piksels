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
pub struct VertexArrayData {
  attrs: Vec<VertexAttr>,
  layout: MemoryLayout,
}

impl VertexArrayData {
  pub fn new(attrs: Vec<VertexAttr>, layout: MemoryLayout) -> Self {
    VertexArrayData { attrs, layout }
  }

  pub fn attrs(&self) -> &[VertexAttr] {
    &self.attrs
  }

  pub fn layout(&self) -> &MemoryLayout {
    &self.layout
  }

  /// Number of elements present in the data.
  pub fn len(&self) -> usize {
    match self.layout {
      // for interleaved memory, we simply compute the size of a vertex by summing the size of all of its attributes,
      // and simply divide the data length by this value
      MemoryLayout::Interleaved { ref data } => {
        let vertex_len: usize = self.attrs.iter().map(VertexAttr::size).sum();

        if vertex_len == 0 {
          0
        } else {
          data.len() / vertex_len
        }
      }

      // for deinterleaved memory, we are supposed to have the same number of vertices in each array, so we can simply just
      // take the data slot, zip it with the first attribute and do the division
      MemoryLayout::Deinterleaved { ref data_per_attr } => data_per_attr
        .first()
        .and_then(|data| self.attrs.first().map(|attr| data.len() / attr.size()))
        .unwrap_or(0),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self.layout {
      MemoryLayout::Interleaved { ref data } => data.is_empty(),
      MemoryLayout::Deinterleaved { ref data_per_attr } => data_per_attr
        .first()
        .map(|data| data.is_empty())
        .unwrap_or(true),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryLayout {
  /// Memory is interleaved; i.e. { x0, y0, r0, g0, b0, x1, y1, r1, g1, b1 }.
  Interleaved { data: Vec<u8> },

  /// Memory is deinterleaved; i.e. { x0, y0, x1, y1 } { r0, g0, b0, r1, g1, b1 }.
  Deinterleaved { data_per_attr: Vec<Vec<u8>> },
}
