//! Meta backend for OpenGL-like APIs.
//!
//! This crate serves as an intermediary compile-time interface for backends implementing an OpenGL-like API, such as
//! OpenGL, OpenGL ES and WebGL.

use buffer::BufferTarget;

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

  fn update_buffer(
    &self,
    target: BufferTarget,
    bytes: &[u8],
    start: usize,
    len: usize,
  ) -> Result<(), Self::Err>;

  fn new_vao(&self) -> Result<Self::Vao, Self::Err>;

  fn bind_vao(&self, vao: &Self::Vao) -> Result<(), Self::Err>;
}

#[derive(Debug)]
pub struct VertexArray<B>
where
  B: OpenGLBackend,
{
  vao: B::Vao,
}
