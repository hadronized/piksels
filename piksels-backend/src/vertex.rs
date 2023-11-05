use std::mem::size_of;

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

  /// Alignment of the vertex attribute.
  pub fn align(&self) -> usize {
    match self.ty {
      Type::Int { size, .. }
      | Type::Uint { size, .. }
      | Type::Int2 { size, .. }
      | Type::Uint2 { size, .. }
      | Type::Int3 { size, .. }
      | Type::Int4 { size, .. }
      | Type::Uint3 { size, .. }
      | Type::Uint4 { size, .. } => size.size(),
      Type::Float
      | Type::Bool
      | Type::Float2
      | Type::Float3
      | Type::Float4
      | Type::Double
      | Type::Double2
      | Type::Double3
      | Type::Double4
      | Type::Bool2
      | Type::Bool3
      | Type::Bool4 => size_of::<u32>(),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Size {
  Eight,
  Sixteen,
  Thirteen,
}

impl Size {
  /// Size in bytes.
  pub fn size(&self) -> usize {
    match self {
      Size::Eight => 1,
      Size::Sixteen => 2,
      Size::Thirteen => 4,
    }
  }
}

/// Possible type of vertex attributes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
  Int { size: Size, normalized: Normalized },
  Int2 { size: Size, normalized: Normalized },
  Int3 { size: Size, normalized: Normalized },
  Int4 { size: Size, normalized: Normalized },
  Uint { size: Size, normalized: Normalized },
  Uint2 { size: Size, normalized: Normalized },
  Uint3 { size: Size, normalized: Normalized },
  Uint4 { size: Size, normalized: Normalized },
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
      Self::Int { size, .. } | Self::Uint { size, .. } => size.size(),
      Self::Int2 { size, .. } | Self::Uint2 { size, .. } => 2 * size.size(),
      Self::Int3 { size, .. } | Self::Uint3 { size, .. } => 3 * size.size(),
      Self::Int4 { size, .. } | Self::Uint4 { size, .. } => 4 * size.size(),
      Self::Float | Self::Bool => 4,
      Self::Float2 | Self::Bool2 => 4 * 2,
      Self::Float3 | Self::Bool3 => 4 * 3,
      Self::Float4 | Self::Bool4 => 4 * 4,
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
      Self::Int2 { .. } | Self::Uint2 { .. } | Self::Float2 | Self::Double2 | Self::Bool2 => 2,
      Self::Int3 { .. } | Self::Uint3 { .. } | Self::Float3 | Self::Double3 | Self::Bool3 => 3,
      Self::Int4 { .. } | Self::Uint4 { .. } | Self::Float4 | Self::Double4 | Self::Bool4 => 4,
      _ => 1,
    }
  }

  /// Normalize a vertex attribute type if it’s integral.
  ///
  /// Return the normalized integer vertex attribute type if non-normalized. Otherwise, return the
  /// vertex attribute type directly.
  pub fn normalize(self) -> Self {
    match self {
      Self::Int {
        normalized: Normalized::No,
        size,
      } => Self::Int {
        normalized: Normalized::Yes,
        size,
      },
      Self::Int2 {
        normalized: Normalized::No,
        size,
      } => Self::Int2 {
        normalized: Normalized::Yes,
        size,
      },
      Self::Int3 {
        normalized: Normalized::No,
        size,
      } => Self::Int3 {
        normalized: Normalized::Yes,
        size,
      },
      Self::Int4 {
        normalized: Normalized::No,
        size,
      } => Self::Int4 {
        normalized: Normalized::Yes,
        size,
      },
      Self::Uint {
        normalized: Normalized::No,
        size,
      } => Self::Uint {
        normalized: Normalized::Yes,
        size,
      },
      Self::Uint2 {
        normalized: Normalized::No,
        size,
      } => Self::Uint2 {
        normalized: Normalized::Yes,
        size,
      },
      Self::Uint3 {
        normalized: Normalized::No,
        size,
      } => Self::Uint3 {
        normalized: Normalized::Yes,
        size,
      },
      Self::Uint4 {
        normalized: Normalized::No,
        size,
      } => Self::Uint4 {
        normalized: Normalized::Yes,
        size,
      },
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
