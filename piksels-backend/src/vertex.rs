#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VertexAttrDesc {
  pub index: usize,
  pub name: &'static str,
  pub ty: Type,
  pub array: Option<usize>,
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
