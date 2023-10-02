/// Scissor mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Scissor {
  Off,
  On(ScissorRegion),
}

/// The region outside of which fragments will be discarded.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScissorRegion {
  /// The x screen position of the scissor region.
  x: u32,

  /// The y screen position of the scissor region.
  y: u32,

  /// The screen width of the scissor region.
  width: u32,

  /// The screen height of the scissor region.
  height: u32,
}
