/// Command buffer that can accept commands.
#[derive(Clone, Debug, PartialEq)]
pub struct CmdBuf {
  handle: usize,
}

mk_bckd_type_getters!(CmdBuf, handle -> usize);
