/// A color image attachment point.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ColorAttachmentPoint {
  index: usize,
  name: &'static str,
  ty: ColorType,
}

mk_bckd_type_getters!(
  ColorAttachmentPoint,
  index -> usize,
  name -> &'static str,
  ty -> ColorType
);

/// A depth-stencil attachment point.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DepthStencilAttachmentPoint {
  index: usize,
  name: &'static str,
  ty: DepthStencilType,
}

mk_bckd_type_getters!(
  DepthStencilAttachmentPoint,
  index -> usize,
  name -> &'static str,
  ty -> DepthStencilType
);

/// Color attachment type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorType {
  /// Integral red channel.
  IR { red_bits: ChannelBits },

  /// Integral red/green channel.
  IRG {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
  },

  /// Integral red/green/blue channel.
  IRGB {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
  },

  /// Integral red/green/blue channel. Linear version.
  ISRGB {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
  },

  /// Integral red/green/blue/alpha channel.
  IRGBA {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
    alpha_bits: ChannelBits,
  },

  /// Integral red/green/blue/alpha channel. Linear version.
  ISRGBA {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
    alpha_bits: ChannelBits,
  },

  /// Unsigned integral red channel.
  UintR { red_bits: ChannelBits },

  /// Unsigned integral red/green channel.
  UintRG {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
  },

  /// Unsigned integral red/green/blue channel.
  UintRGB {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
  },

  /// Unsigned integral red/green/blue channel. Linear version.
  UintSRGB {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
  },

  /// Unsigned integral red/green/blue/alpha channel.
  UintRGBA {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
    alpha_bits: ChannelBits,
  },

  UintSRGBA {
    red_bits: ChannelBits,
    green_bits: ChannelBits,
    blue_bits: ChannelBits,
    alpha_bits: ChannelBits,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DepthStencilType {
  Depth {
    depth_bits: ChannelBits,
  },

  DepthStencil {
    depth_bits: ChannelBits,
    stencil_bits: ChannelBits,
  },
}

/// Size in bits a pixel channel can be.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
