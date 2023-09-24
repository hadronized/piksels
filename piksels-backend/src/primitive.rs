/// Primitive class, like points, lines, triangles, etc.
pub trait Primitive {
  /// Connector used to link vertices together, if any.
  const CONNECTOR: Connector;
}

/// Connector used to interpret vertices in vertex arrays.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Connector {
  /// Vertices are not connected and are considered as points.
  Point,

  /// Vertices are connected two by two as disjoint lines; i.e. 0-1, 2-3, 4-5, etc.
  Line,

  /// Vertices are connected as a continuous stroke line, starting with the first two vertices, and then each new vertex
  /// in the stream extends the line; i.e. 0-1, 1-2, 2-3, 3-4, etc.
  LineStrip,

  /// Vertices are connected three by three as disjoint triangles; i.e. 0-1-2, 3-4-5, 6-7-8, etc.
  Triangle,

  /// Vertices are connected as a fold of triangles, starting with the first three vertices, and then each new vertex
  /// in the stream creates a new triangle by taking the last two vertices; i.e. 0-1-2, 1-2-3, 2-3-4, 3-4-5, etc.
  TriangleStrip,

  /// Vertices are connected as a fold of triangles, starting with the first three vertices, and then each new vertex
  /// in the stream creates a new triangle by taking the first and last vertices; i.e. 0-1-2, 0-2-3, 0-3-4, 0-4-5, etc.
  TriangleFan,

  /// Vertices are grouped together and the actual primitive connector is dynamically computed in a shader.
  Patch(usize),
}
