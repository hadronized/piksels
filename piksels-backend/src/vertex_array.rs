use crate::vertex::VertexAttr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArrayData {
  attrs: Vec<VertexAttr>,
  layout: MemoryLayout<Vec<u8>>,
}

impl VertexArrayData {
  pub fn new(attrs: Vec<VertexAttr>, layout: MemoryLayout<Vec<u8>>) -> Self {
    VertexArrayData { attrs, layout }
  }

  pub fn attrs(&self) -> &[VertexAttr] {
    &self.attrs
  }

  pub fn layout(&self) -> &MemoryLayout<Vec<u8>> {
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
pub enum MemoryLayout<T> {
  /// Memory is interleaved; i.e. { x0, y0, r0, g0, b0, x1, y1, r1, g1, b1 }.
  Interleaved { data: T },

  /// Memory is deinterleaved; i.e. { x0, y0, x1, y1 } { r0, g0, b0, r1, g1, b1 }.
  Deinterleaved { data_per_attr: Vec<T> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataSelector {
  /// Select interleaved vertices.
  InterleavedVertices,

  /// Select interleaved vertex instances.
  InterleavedVertexInstances,

  /// Select deinterleaved vertices (field identified by `index`).
  DeinterleavedVertices { index: usize },

  /// Select deinterleaved vertex instances (field identified by `index`).
  DeinterleavedVertexInstances { index: usize },

  /// Select indices.
  Indices,
}
