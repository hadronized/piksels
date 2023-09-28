#[derive(Debug, Eq, PartialEq)]
pub struct Shader {
  handle: usize,
}

mk_bckd_type_getters!(Shader, handle -> usize);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShaderSources {
  tess_ctrl_stage: String,
  tess_eval_stage: String,
  vertex_stage: String,
  geometry_stage: String,
  fragment_stage: String,
}

impl ShaderSources {
  pub fn tess_ctrl(mut self, tess_ctrl_stage: impl Into<String>) -> Self {
    self.tess_ctrl_stage = tess_ctrl_stage.into();
    self
  }

  pub fn tess_eval(mut self, tess_eval_stage: impl Into<String>) -> Self {
    self.tess_eval_stage = tess_eval_stage.into();
    self
  }

  pub fn vertex(mut self, vertex_stage: impl Into<String>) -> Self {
    self.vertex_stage = vertex_stage.into();
    self
  }

  pub fn geometry(mut self, geometry_stage: impl Into<String>) -> Self {
    self.geometry_stage = geometry_stage.into();
    self
  }

  pub fn fragment(mut self, fragment_stage: impl Into<String>) -> Self {
    self.fragment_stage = fragment_stage.into();
    self
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Uniform {
  index: usize,
  ty: UniformType,
}

mk_bckd_type_getters!(Uniform, index -> usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UniformType {
  base: UniformTypeBase,
  array: Option<usize>,
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

#[derive(Debug, Eq, PartialEq)]
pub struct UniformBuffer {
  handle: usize,
}

mk_bckd_type_getters!(UniformBuffer, handle -> usize);
