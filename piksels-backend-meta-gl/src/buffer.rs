/// Buffer targets for [`Buffer`] — e.g. when binding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BufferTarget {
  Array,
  Elements,
  Uniform,
}
