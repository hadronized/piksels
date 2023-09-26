/// A framebuffer is a gathering of image attachments.
#[derive(Debug, Eq, PartialEq)]
pub struct Framebuffer {
  handle: usize,
}

mk_bck_type_getters!(Framebuffer, handle -> usize);

/// A color image attachment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ColorAttachment {
  index: usize,
  ty: ColorType,
}

mk_bck_type_getters!(ColorAttachment, index -> usize);

/// A depth-stencil attachment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DepthStencilAttachment {
  index: usize,
  ty: DepthStencilType,
}

mk_bck_type_getters!(DepthStencilAttachment, index -> usize);

/// Color attachment type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorType {
  /// Integral red channel.
  IR { red_bits: u8 },

  /// Integral red/green channel.
  IRG { red_bits: u8, green_bits: u8 },

  /// Integral red/green/blue channel.
  IRGB {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
  },

  /// Integral red/green/blue channel. Linear version.
  ISRGB {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
  },

  /// Integral red/green/blue/alpha channel.
  IRGBA {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
    alpha_bits: u8,
  },

  /// Integral red/green/blue/alpha channel. Linear version.
  ISRGBA {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
    alpha_bits: u8,
  },

  /// Unsigned integral red channel.
  UintR { red_bits: u8 },

  /// Unsigned integral red/green channel.
  UintRG { red_bits: u8, green_bits: u8 },

  /// Unsigned integral red/green/blue channel.
  UintRGB {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
  },

  /// Unsigned integral red/green/blue channel. Linear version.
  UintSRGB {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
  },

  /// Unsigned integral red/green/blue/alpha channel.
  UintRGBA {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
    alpha_bits: u8,
  },

  UintSRGBA {
    red_bits: u8,
    green_bits: u8,
    blue_bits: u8,
    alpha_bits: u8,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DepthStencilType {
  Depth {
    depth_bits: usize,
  },

  DepthStencil {
    depth_bits: usize,
    stencil_bits: usize,
  },
}
