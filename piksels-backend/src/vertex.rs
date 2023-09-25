#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VertexAttr {
  pub index: usize,
  pub name: &'static str,
  pub ty: Type,
  pub array: Option<usize>,
}

impl VertexAttr {
  /// Size in bytes of a vertex attribute.
  pub fn size(&self) -> usize {
    self.ty.size() * self.array.unwrap_or(1)
  }
}

/// Possible type of vertex attributes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
  Int(Normalized),
  Int2(Normalized),
  Int3(Normalized),
  Int4(Normalized),
  Uint(Normalized),
  Uint2(Normalized),
  Uint3(Normalized),
  Uint4(Normalized),
  Float,
  Float2,
  Float3,
  Float4,
  Double,
  Double2,
  Double3,
  Double4,
  Bool,
  Bool2,
  Bool3,
  Bool4,
}

impl Type {
  /// Size in bytes.
  pub fn size(&self) -> usize {
    match self {
      Self::Int(_) | Self::Uint(_) | Self::Float | Self::Bool => 4,
      Self::Int2(_) | Self::Uint2(_) | Self::Float2 | Self::Bool2 => 4 * 2,
      Self::Int3(_) | Self::Uint3(_) | Self::Float3 | Self::Bool3 => 4 * 3,
      Self::Int4(_) | Self::Uint4(_) | Self::Float4 | Self::Bool4 => 4 * 4,
      Self::Double => 8,
      Self::Double2 => 8 * 2,
      Self::Double3 => 8 * 3,
      Self::Double4 => 8 * 4,
    }
  }

  /// Vector dimension.
  ///
  /// This makes sense only for vectors. Scalars always have a dimension of `1`.
  pub fn vector_dim(&self) -> usize {
    match self {
      Self::Int2(_) | Self::Uint2(_) | Self::Float2 | Self::Double2 | Self::Bool2 => 2,
      Self::Int3(_) | Self::Uint3(_) | Self::Float3 | Self::Double3 | Self::Bool3 => 3,
      Self::Int4(_) | Self::Uint4(_) | Self::Float4 | Self::Double4 | Self::Bool4 => 4,
      _ => 1,
    }
  }

  /// Normalize a vertex attribute type if it’s integral.
  ///
  /// Return the normalized integer vertex attribute type if non-normalized. Otherwise, return the
  /// vertex attribute type directly.
  pub fn normalize(self) -> Self {
    match self {
      Self::Int(Normalized::No) => Self::Int(Normalized::Yes),
      Self::Int2(Normalized::No) => Self::Int2(Normalized::Yes),
      Self::Int3(Normalized::No) => Self::Int3(Normalized::Yes),
      Self::Int4(Normalized::No) => Self::Int4(Normalized::Yes),
      Self::Uint(Normalized::No) => Self::Uint(Normalized::Yes),
      Self::Uint2(Normalized::No) => Self::Uint2(Normalized::Yes),
      Self::Uint3(Normalized::No) => Self::Uint3(Normalized::Yes),
      Self::Uint4(Normalized::No) => Self::Uint4(Normalized::Yes),
      _ => self,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Normalized {
  /// Normalize integral values and expose them as floating-point values.
  Yes,

  /// Do not perform any normalization and hence leave integral values as-is.
  No,
}
