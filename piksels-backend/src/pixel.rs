/// Gathers a pixel type along with a format.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pixel {
  pub encoding: Type,
  pub format: Format,
}

impl Pixel {
  /// Does a [`PixelFormat`] represent a color?
  pub fn is_color_pixel(self) -> bool {
    !matches!(self.format, Format::Depth(_))
  }

  /// Does a [`PixelFormat`] represent depth information?
  pub fn is_depth_pixel(self) -> bool {
    !self.is_color_pixel()
  }

  /// Return the number of channels.
  pub fn channels_len(self) -> usize {
    match self.format {
      Format::R(_) => 1,
      Format::RG(..) => 2,
      Format::RGB(..) => 3,
      Format::RGBA(..) => 4,
      Format::SRGB(..) => 3,
      Format::SRGBA(..) => 4,
      Format::Depth(_) => 1,
      Format::DepthStencil(..) => 2,
    }
  }
}

/// Pixel type.
///
/// - Normalized integer types: [`NormIntegral`] and [`NormUnsigned`] represent integer types
///   (signed and unsigned, respectively). However, they are _normalized_ when used in shader
///   stages, i.e. fetching from them will yield a floating-point value. That value is
///   comprised between `0.0` and `1.0`.
/// - Integer types: [`Integral`] and [`Unsigned`] allows to store signed and unsigned integers,
///   respectively.
/// - Floating-point types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
  /// Normalized signed integral pixel type.
  NormIntegral,

  /// Normalized unsigned integral pixel type.
  NormUnsigned,

  /// Signed integral pixel type.
  Integral,

  /// Unsigned integral pixel type.
  Unsigned,

  /// Floating-point pixel type.
  Floating,
}

/// Format of a pixel.
///
/// Whichever the constructor you choose, the carried value represent how many bits are used to
/// represent each channel.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Format {
  /// Holds a red-only channel.
  R(ChannelBits),

  /// Holds red and green channels.
  RG(ChannelBits, ChannelBits),

  /// Holds red, green and blue channels.
  RGB(ChannelBits, ChannelBits, ChannelBits),

  /// Holds red, green, blue and alpha channels.
  RGBA(ChannelBits, ChannelBits, ChannelBits, ChannelBits),

  /// Holds a red, green and blue channels in sRGB colorspace.
  SRGB(ChannelBits, ChannelBits, ChannelBits),

  /// Holds a red, green and blue channels in sRGB colorspace, plus an alpha channel.
  SRGBA(ChannelBits, ChannelBits, ChannelBits, ChannelBits),

  /// Holds a depth channel.
  Depth(ChannelBits),

  /// Holds a depth+stencil channel.
  DepthStencil(ChannelBits, ChannelBits),
}

impl Format {
  /// Size (in bytes) of a pixel that a format represents.
  pub fn bytes(self) -> usize {
    let bits = match self {
      Format::R(r) => r.bits(),
      Format::RG(r, g) => r.bits() + g.bits(),
      Format::RGB(r, g, b) => r.bits() + g.bits() + b.bits(),
      Format::RGBA(r, g, b, a) => r.bits() + g.bits() + b.bits() + a.bits(),
      Format::SRGB(r, g, b) => r.bits() + g.bits() + b.bits(),
      Format::SRGBA(r, g, b, a) => r.bits() + g.bits() + b.bits() + a.bits(),
      Format::Depth(d) => d.bits(),
      Format::DepthStencil(d, s) => d.bits() + s.bits(),
    };

    bits / 8
  }
}

/// Size in bits a pixel channel can be.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ChannelBits {
  /// 8-bit.
  Eight,

  /// 10-bit.
  Ten,

  /// 11-bit.
  Eleven,

  /// 16-bit.
  Sixteen,

  /// 32-bit.
  ThirtyTwo,
}

impl ChannelBits {
  /// Size (in bits).
  pub fn bits(self) -> usize {
    match self {
      ChannelBits::Eight => 8,
      ChannelBits::Ten => 10,
      ChannelBits::Eleven => 11,
      ChannelBits::Sixteen => 16,
      ChannelBits::ThirtyTwo => 32,
    }
  }
}
