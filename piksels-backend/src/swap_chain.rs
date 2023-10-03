/// Swap chain mode to use with a swap chain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SwapChainMode {
  /// Immediately transfer image data to the physical in-use memory.
  Immediate,

  /// The swap chain acts as a FIFO between the physical in-use memory and rendered targets. An image is polled from the
  /// front of the FIFO every time a V-blank is reached and a new image is added at the end of the queue.
  Fifo,

  /// Same thing as [`SwapChainMode::Fifo`] but whenever the FIFO is full, new images replaces old ones.
  Mailbox,
}
