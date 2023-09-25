use std::ops::{Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive};

use piksels_backend::vertex_array::VertexArrayData;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArray {
  raw: piksels_backend::vertex_array::VertexArray,
  vertices: VertexArrayData,
  instances: VertexArrayData,
  indices: Vec<u32>,
  vertex_count: usize,
}

impl VertexArray {
  /// Number of vertices.
  pub fn vertex_count(&self) -> usize {
    self.vertex_count
  }

  pub fn vertices(&mut self) -> &mut VertexArrayData {
    &mut self.vertices
  }

  pub fn instances(&mut self) -> &mut VertexArrayData {
    &mut self.instances
  }

  pub fn indices(&mut self) -> &mut Vec<u32> {
    &mut self.indices
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArrayView {
  handle: usize,
  start_vertex: usize,
  vertex_count: usize,
  instance_count: usize,
}

impl VertexArrayView {
  pub fn new(va: &VertexArray) -> Self {
    let handle = va.raw.handle();
    let vertex_count = va.vertex_count;

    Self {
      handle,
      start_vertex: 0,
      vertex_count,
      instance_count: 1,
    }
  }

  pub fn handle(&self) -> usize {
    self.handle
  }

  pub fn start_vertex(&self) -> usize {
    self.start_vertex
  }

  pub fn set_start_vertex(mut self, start_vertex: usize) -> Self {
    self.start_vertex = start_vertex;
    self
  }

  pub fn vertex_count(&self) -> usize {
    self.vertex_count
  }

  pub fn set_vertex_count(mut self, vertex_count: usize) -> Self {
    self.vertex_count = vertex_count;
    self
  }

  pub fn instance_count(&self) -> usize {
    self.instance_count
  }

  pub fn set_instance_count(mut self, instance_count: usize) -> Self {
    self.instance_count = instance_count;
    self
  }
}

pub trait View<R> {
  fn view(&self, range: R) -> VertexArrayView;
}

impl View<RangeFull> for VertexArray {
  fn view(&self, _: RangeFull) -> VertexArrayView {
    VertexArrayView::new(self)
  }
}

impl View<Range<usize>> for VertexArray {
  fn view(&self, range: Range<usize>) -> VertexArrayView {
    VertexArrayView {
      handle: self.raw.handle(),
      start_vertex: range.start,
      vertex_count: range.end,
      instance_count: 1,
    }
  }
}

impl View<RangeFrom<usize>> for VertexArray {
  fn view(&self, range: RangeFrom<usize>) -> VertexArrayView {
    VertexArrayView {
      handle: self.raw.handle(),
      start_vertex: range.start,
      vertex_count: self.vertex_count - range.start,
      instance_count: 1,
    }
  }
}

impl View<RangeTo<usize>> for VertexArray {
  fn view(&self, range: RangeTo<usize>) -> VertexArrayView {
    VertexArrayView {
      handle: self.raw.handle(),
      start_vertex: 0,
      vertex_count: range.end - 1,
      instance_count: 1,
    }
  }
}

impl View<RangeToInclusive<usize>> for VertexArray {
  fn view(&self, range: RangeToInclusive<usize>) -> VertexArrayView {
    VertexArrayView {
      handle: self.raw.handle(),
      start_vertex: 0,
      vertex_count: range.end,
      instance_count: 1,
    }
  }
}
