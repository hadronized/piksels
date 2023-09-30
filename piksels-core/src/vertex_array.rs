use std::ops::{Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive};

use piksels_backend::{
  vertex_array::{VertexArrayData, VertexArrayUpdate},
  Backend,
};

#[derive(Debug, Eq, PartialEq)]
pub struct VertexArray<B>
where
  B: Backend,
{
  pub(crate) raw: B::VertexArray,
  vertices: VertexArrayData,
  instances: VertexArrayData,
  indices: Vec<u32>,
  vertex_count: usize,
}

impl<B> Drop for VertexArray<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_vertex_array(&self.raw);
  }
}

impl<B> VertexArray<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(
    raw: B::VertexArray,
    vertices: VertexArrayData,
    instances: VertexArrayData,
    indices: Vec<u32>,
  ) -> Self {
    let vertex_count = if indices.is_empty() {
      vertices.len()
    } else {
      indices.len()
    };

    VertexArray {
      raw,
      vertices,
      instances,
      indices,
      vertex_count,
    }
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

  pub fn vertex_count(&self) -> usize {
    self.vertex_count
  }

  pub fn update(&self, update: VertexArrayUpdate) -> Result<(), B::Err> {
    B::update_vertex_array(&self.raw, update)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArrayView<'a, B>
where
  B: Backend,
{
  vertex_array: &'a B::VertexArray,
  start_vertex: usize,
  vertex_count: usize,
  instance_count: usize,
}

impl<'a, B> VertexArrayView<'a, B>
where
  B: Backend,
{
  pub fn vertex_array(&self) -> &'a B::VertexArray {
    self.vertex_array
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

/// A helper trait to obtain a [`VertexArrayView`] from a [`VertexArray`].
pub trait View<B, R>
where
  B: Backend,
{
  fn view(&self, range: R) -> VertexArrayView<B>;
}

impl<B> View<B, RangeFull> for VertexArray<B>
where
  B: Backend,
{
  fn view(&self, _: RangeFull) -> VertexArrayView<B> {
    VertexArrayView {
      vertex_array: &self.raw,
      start_vertex: 0,
      vertex_count: self.vertex_count,
      instance_count: 1,
    }
  }
}

impl<B> View<B, Range<usize>> for VertexArray<B>
where
  B: Backend,
{
  fn view(&self, range: Range<usize>) -> VertexArrayView<B> {
    VertexArrayView {
      vertex_array: &self.raw,
      start_vertex: range.start,
      vertex_count: range.end,
      instance_count: 1,
    }
  }
}

impl<B> View<B, RangeFrom<usize>> for VertexArray<B>
where
  B: Backend,
{
  fn view(&self, range: RangeFrom<usize>) -> VertexArrayView<B> {
    VertexArrayView {
      vertex_array: &self.raw,
      start_vertex: range.start,
      vertex_count: self.vertex_count - range.start,
      instance_count: 1,
    }
  }
}

impl<B> View<B, RangeTo<usize>> for VertexArray<B>
where
  B: Backend,
{
  fn view(&self, range: RangeTo<usize>) -> VertexArrayView<B> {
    VertexArrayView {
      vertex_array: &self.raw,
      start_vertex: 0,
      vertex_count: range.end - 1,
      instance_count: 1,
    }
  }
}

impl<B> View<B, RangeToInclusive<usize>> for VertexArray<B>
where
  B: Backend,
{
  fn view(&self, range: RangeToInclusive<usize>) -> VertexArrayView<B> {
    VertexArrayView {
      vertex_array: &self.raw,
      start_vertex: 0,
      vertex_count: range.end,
      instance_count: 1,
    }
  }
}
