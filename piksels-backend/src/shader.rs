#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShaderSources<'a> {
  tess_ctrl_stage: &'a str,
  tess_eval_stage: &'a str,
  vertex_stage: &'a str,
  geometry_stage: &'a str,
  fragment_stage: &'a str,
}

impl<'a> ShaderSources<'a> {
  pub fn tess_ctrl(mut self, tess_ctrl_stage: &'a str) -> Self {
    self.tess_ctrl_stage = tess_ctrl_stage;
    self
  }

  pub fn tess_eval(mut self, tess_eval_stage: &'a str) -> Self {
    self.tess_eval_stage = tess_eval_stage;
    self
  }

  pub fn vertex(mut self, vertex_stage: &'a str) -> Self {
    self.vertex_stage = vertex_stage;
    self
  }

  pub fn geometry(mut self, geometry_stage: &'a str) -> Self {
    self.geometry_stage = geometry_stage;
    self
  }

  pub fn fragment(mut self, fragment_stage: &'a str) -> Self {
    self.fragment_stage = fragment_stage;
    self
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UniformType {
  base: UniformTypeBase,
  array: Option<usize>,
}

impl From<UniformTypeBase> for UniformType {
  fn from(base: UniformTypeBase) -> Self {
    Self { base, array: None }
  }
}

impl UniformType {
  pub fn new(base: UniformTypeBase, array: impl Into<Option<usize>>) -> Self {
    Self {
      base,
      array: array.into(),
    }
  }

  pub fn array(mut self, array: usize) -> Self {
    self.array = Some(array);
    self
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UniformTypeBase {
  Int,
  Int2,
  Int3,
  Int4,
  Uint,
  Uint2,
  Uint3,
  Uint4,
  Bool,
  Bool2,
  Bool3,
  Bool4,
  Float,
  Float2,
  Float3,
  Float4,
  Double,
  Double2,
  Double3,
  Double4,
  FloatMat22,
  FloatMat23,
  FloatMat24,
  FloatMat32,
  FloatMat33,
  FloatMat34,
  FloatMat42,
  FloatMat43,
  FloatMat44,
  DoubleMat22,
  DoubleMat23,
  DoubleMat24,
  DoubleMat32,
  DoubleMat33,
  DoubleMat34,
  DoubleMat42,
  DoubleMat43,
  DoubleMat44,
  // TODO: texture types
  // TODO: shader storage types (like UBO, SSBO, etc.?); -> buffer
}
