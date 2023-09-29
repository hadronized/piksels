#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Viewport {
  /// The whole viewport is used. The position and dimension of the viewport rectangle are
  /// extracted from the framebuffer.
  Whole,

  /// The viewport is specific and the rectangle area is user-defined.
  Specific {
    /// The lower position on the X axis to start the viewport rectangle at.
    x: u32,

    /// The lower position on the Y axis to start the viewport rectangle at.
    y: u32,

    /// The width of the viewport.
    width: u32,

    /// The height of the viewport.
    height: u32,
  },
}
