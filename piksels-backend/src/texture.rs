use crate::depth_stencil::Comparison;

/// How to wrap texture coordinates while sampling textures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Wrap {
  /// If textures coordinates lay outside of `[0;1]`, they will be clamped to either `0` or `1` for
  /// every components.
  ClampToEdge,

  /// Textures coordinates are repeated if they lay outside of `[0;1]`. Picture this as:
  ///
  /// ```ignore
  /// // given the frac function returning the fractional part of a floating number:
  /// coord_ith = frac(coord_ith); // always between `[0;1]`
  /// ```
  Repeat,

  /// Same as `Repeat` but it will alternatively repeat between `[0;1]` and `[1;0]` depending on the oddity of the
  /// non-fractional part.
  MirroredRepeat,
}

/// Minification filter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MinFilter {
  /// Nearest interpolation (closest texel value).
  ///
  /// It’s the cheapest kind of filtering.
  Nearest,

  /// Linear interpolation between surrounding texels.
  Linear,

  /// This filter will select the nearest mipmap between two samples and will perform a nearest
  /// interpolation afterwards.
  NearestMipmapNearest,

  /// This filter will select the nearest mipmap between two samples and will perform a linear
  /// interpolation afterwards.
  NearestMipmapLinear,

  /// This filter will linearly interpolate between two mipmaps, which selected texels would have
  /// been interpolated with a nearest filter.
  LinearMipmapNearest,

  /// This filter will linearly interpolate between two mipmaps, which selected texels would have
  /// been linarily interpolated as well.
  ///
  /// It’s the most costly kind of filtering.
  LinearMipmapLinear,
}

/// Magnification filter.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MagFilter {
  /// Nearest interpolation.
  Nearest,

  /// Linear interpolation between surrounding pixels.
  Linear,
}

/// A [`Sampler`] object gives hint on how a [`Texture`] should be sampled.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Sampling {
  /// How should we wrap around the _r_ sampling coordinate?
  pub wrap_r: Wrap,

  /// How should we wrap around the _s_ sampling coordinate?
  pub wrap_s: Wrap,

  /// How should we wrap around the _t_ sampling coordinate?
  pub wrap_t: Wrap,

  /// Minification filter.
  pub min_filter: MinFilter,

  /// Magnification filter.
  pub mag_filter: MagFilter,

  /// For depth textures, should we perform depth comparison and if so, how?
  pub depth_comparison: Option<Comparison>,
}

/// Texture storage data.
///
/// A texture can be flat or layered. Flat textures hold a single collection of texels in each of their mipmaps. Layered
/// textures, on the other side, hold one or many collection of texels in each of their layers. You can think of layered
/// textures as arrays of textures, basically.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Storage {
  /// 1D texture.
  ///
  /// The texture will have a `x` coordinate and `w` (`width`) dimension.
  Flat1D { with: u32 },

  /// 2D texture.
  ///
  /// The texture will have a `x` and `y` coordinates and w (`width`) and h (`height`) dimensions.
  Flat2D { width: u32, height: u32 },

  /// Multisample 2D texture.
  Flat2DMultiSample {
    width: u32,
    height: u32,
    samples: u32,
  },

  /// 3D texture.
  ///
  /// The texture will have a `x`, `y` and `z` coordinates and w (`width`), h (`height`) and d (`depth`) dimensions.
  Flat3D { width: u32, height: u32, depth: u32 },

  /// Cubemap texture.
  ///
  /// The texture will have a `x` and `y` coordinates, along with a face coordinate, and a `s` (`size`) dimension. Each
  /// face of the cubemap are squared (`w = s` and `h = s`).
  FlatCubemap { size: u32 },

  /// Layered 1D texture.
  Layered1D { width: u32, layers: u32 },

  /// Layered 2D texture.
  Layered2D {
    width: u32,
    height: u32,
    layers: u32,
  },

  /// Layered 2D texture.
  Layered2DMultiSample {
    width: u32,
    height: u32,
    layers: u32,
  },

  /// Layered cubemap texture.
  LayeredCubemap { size: u32, layers: u32 },
}

/// Cube face of a cubemap.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CubeFace {
  /// +X face.
  PosX,

  /// -X face.
  NegX,

  /// +Y face.
  PosY,

  /// -Y face
  NegY,

  /// +Z face.
  PosZ,

  /// -Z face.
  NegZ,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Size {
  Dim1 { width: u32 },

  Dim2 { width: u32, height: u32 },

  Dim3 { width: u32, height: u32, depth: u32 },

  Cubemap { size: u32 },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Offset {
  Dim1 { x: u32 },

  Dim2 { x: u32, y: u32 },

  Dim3 { x: u32, y: u32, z: u32 },

  Cubemap { x: u32, y: u32, face: CubeFace },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
  offset: Offset,
  size: Size,
}
