/// Blending equation. Used to state how blending factors and pixel data should be blended.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Equation {
  /// `Additive` represents the following blending equation:
  ///
  /// > `blended = src * srcK + dst * dstK`
  Additive,

  /// `Subtract` represents the following blending equation:
  ///
  /// > `blended = src * srcK - dst * dstK`
  Subtract,

  /// Because subtracting is not commutative, `ReverseSubtract` represents the following additional
  /// blending equation:
  ///
  /// > `blended = dst * dstK - src * srcK`
  ReverseSubtract,

  /// `Min` represents the following blending equation:
  ///
  /// > `blended = min(src, dst)`
  Min,

  /// `Max` represents the following blending equation:
  ///
  /// > `blended = max(src, dst)`
  Max,
}

/// Blending factors. Pixel data are multiplied by these factors to achieve several effects driven
/// by *blending equations*.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Factor {
  /// `1 * color = color`
  One,

  /// `0 * color = 0`
  Zero,

  /// `src * color`
  SrcColor,

  /// `(1 - src) * color`
  SrcColorComplement,

  /// `dst * color`
  DestColor,

  /// `(1 - dst) * color`
  DestColorComplement,

  /// `srcA * color`
  SrcAlpha,

  /// `(1 - src) * color`
  SrcAlphaComplement,

  /// `dstA * color`
  DstAlpha,

  /// `(1 - dstA) * color`
  DstAlphaComplement,

  /// For colors, `min(srcA, 1 - dstA)`, for alpha, `1`
  SrcAlphaSaturate,
}

/// Basic blending configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Blending {
  /// Blending equation to use.
  pub equation: Equation,

  /// Source factor.
  pub src: Factor,

  /// Destination factor.
  pub dst: Factor,
}

/// Blending configuration to represent combined or separate options.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlendingMode {
  /// Blending is disabled.
  Off,

  /// Blending with combined RGBA.
  Combined(Blending),

  /// Blending with RGB and alpha separately.
  Separate {
    /// Blending configuration for RGB components.
    rgb: Blending,

    /// Blending configuration for alpha component.
    alpha: Blending,
  },
}
