use std::{
  marker::PhantomData,
  ops::{Deref, DerefMut, Range, RangeFrom, RangeFull, RangeTo, RangeToInclusive},
};

use piksels_backend::{vertex_array::DataSelector, Backend};

#[derive(Debug)]
pub struct VertexArray<B>
where
  B: Backend,
{
  pub(crate) raw: B::VertexArray,
  vertex_count: usize,
}

impl<B> VertexArray<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::VertexArray, vertex_count: usize) -> Self {
    Self { raw, vertex_count }
  }

  pub fn map(&self, data_selector: DataSelector) -> Result<VertexArrayMappedBytes<B>, B::Err> {
    B::map_vertex_array_bytes(&self.raw, data_selector).map(VertexArrayMappedBytes::from_raw)
  }

  pub fn vertex_count(&self) -> usize {
    self.vertex_count
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexArrayMappedBytes<'a, B>
where
  B: Backend,
{
  raw: B::VertexArrayMappedBytes,
  _phantom: PhantomData<&'a mut ()>,
}

impl<'a, B> Drop for VertexArrayMappedBytes<'a, B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::unmap_vertex_array_bytes(&self.raw);
  }
}

impl<'a, B> Deref for VertexArrayMappedBytes<'a, B>
where
  B: Backend,
{
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    let (data, len) = B::vertex_array_bytes_data(&self.raw);
    unsafe { std::slice::from_raw_parts(data, len) }
  }
}

impl<'a, B> DerefMut for VertexArrayMappedBytes<'a, B>
where
  B: Backend,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    let (data, len) = B::vertex_array_bytes_data_mut(&mut self.raw);
    unsafe { std::slice::from_raw_parts_mut(data, len) }
  }
}

impl<'a, B> VertexArrayMappedBytes<'a, B>
where
  B: Backend,
{
  fn from_raw(raw: B::VertexArrayMappedBytes) -> Self {
    Self {
      raw,
      _phantom: PhantomData,
    }
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
