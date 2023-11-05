//! Meta backend for OpenGL-like APIs.
//!
//! This crate serves as an intermediary compile-time interface for backends implementing an OpenGL-like API, such as
//! OpenGL, OpenGL ES and WebGL.

use buffer::BufferTarget;
use piksels_backend::{
  extension::{self, ExtensionsBuilder},
  vertex::VertexAttr,
  vertex_array::{DataSelector, MemoryLayout, VertexArrayData},
  Backend, BackendInfo,
};

pub mod buffer;

/// OpenGL-like backend.
///
/// OpenGL-like backends should implement this trait to automatically implement the [`Backend`].
pub trait OpenGLBackend {
  type Err;

  /// Buffer.
  ///
  /// Mainly used as memory store for vertices, vertex instance data and indices.
  type Buffer;

  type Vao;

  fn new_buffer(&self, bytes: &[u8]) -> Result<Self::Buffer, Self::Err>;

  fn bind_buffer(&self, buffer: &Self::Buffer, target: BufferTarget) -> Result<(), Self::Err>;

  fn map_buffer(
    &self,
    target: BufferTarget,
    start: usize,
    len: usize,
  ) -> Result<*mut u8, Self::Err>;

  fn new_vao(&self) -> Result<Self::Vao, Self::Err>;

  fn bind_vao(&self, vao: &Self::Vao) -> Result<(), Self::Err>;
}

#[derive(Debug)]
pub struct OpenGLLike<B> {
  backend: B,
}

impl<B> OpenGLLike<B> {
  pub fn new(backend: B) -> Self {
    Self { backend }
  }
}

#[derive(Debug)]
pub struct VertexArray<B>
where
  B: OpenGLBackend,
{
  vao: B::Vao,
  vertex_buffers: Option<MemoryLayout<B::Buffer>>,
  instance_buffers: Option<MemoryLayout<B::Buffer>>,
  index_buffer: Option<B::Buffer>,
}

impl<B> VertexArray<B>
where
  B: OpenGLBackend,
{
  fn new_vertex_array(
    backend: &B,
    vertices: Option<&VertexArrayData>,
    instances: Option<&VertexArrayData>,
    indices: Option<&[u32]>,
  ) -> Result<Self, B::Err> {
    let vao = backend.new_vao()?;
    backend.bind_vao(&vao)?;

    // TODO: build vertex buffers, if any
    // TODO: build instance buffers, if any
    // TODO: build indices buffer, if any
    todo!()
  }

  fn build_vertex_buffers(
    backend: &B,
    data: Option<&VertexArrayData>,
    instanced: bool,
  ) -> Result<Option<MemoryLayout<B::Buffer>>, B::Err> {
    match data {
      None => Ok(None),
      Some(vad) => match vad.layout() {
        MemoryLayout::Interleaved { data } => todo!(),
        MemoryLayout::Deinterleaved { data_per_attr } => todo!(),
      },
    }
  }

  fn build_interleaved_buffer(
    backend: &B,
    attrs: &[VertexAttr],
    data: &[u8],
    instanced: bool,
  ) -> Result<Option<MemoryLayout<B::Buffer>>, B::Err> {
    if data.is_empty() {
      // no need to create a vertex buffer
      return Ok(None);
    }

    let buf = backend.new_buffer(data)?;
    Self::set_vertex_pointers(backend, attrs, instanced)?;

    todo!()
  }

  fn set_vertex_pointers(backend: &B, attrs: &[VertexAttr], instanced: bool) -> Result<(), B::Err> {
    // This function sets the vertex attribute pointers for the input list by computing:
    //
    // - The stride: this is easily computed, since it’s the size (bytes) of a single vertex.
    // - The offsets: each attribute has a given offset in the buffer. This is computed by
    //   accumulating the size of all previously set attributes and aligning offsets.
    let stride = attrs.iter().map(VertexAttr::size).sum::<usize>();
    let offsets = todo!();
    todo!()
  }

  fn aligned_offsets(attrs: &[VertexAttr]) -> impl Iterator<Item = usize> {
    let mut off = 0;

    attrs.iter().map(|attr| {
      off = Self::off_align(off, attr.align());
      off += Self::attr
    })
  }

  /// Align an offset.
  #[inline]
  fn off_align(off: usize, align: usize) -> usize {
    let a = align - 1;
    (off + a) & !a
  }
}
