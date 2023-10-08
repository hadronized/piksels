use piksels_backend::{shader::UniformType, Backend};

#[derive(Debug, Eq, PartialEq)]
pub struct Shader<B>
where
  B: Backend,
{
  pub(crate) raw: B::Shader,
}

impl<B> Drop for Shader<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_shader(&self.raw);
  }
}

impl<B> Shader<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::Shader) -> Self {
    Self { raw }
  }

  pub fn uniform(
    &self,
    name: impl AsRef<str>,
    ty: impl Into<UniformType>,
  ) -> Result<Uniform<B>, B::Err> {
    B::get_uniform(&self.raw, name.as_ref(), ty.into()).map(|raw| Uniform { raw })
  }

  pub fn uniform_buffer(&self, name: impl AsRef<str>) -> Result<UniformBuffer<B>, B::Err> {
    B::get_uniform_buffer(&self.raw, name.as_ref()).map(|raw| UniformBuffer { raw })
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Uniform<B>
where
  B: Backend,
{
  pub(crate) raw: B::Uniform,
}

impl<B> Drop for Uniform<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_uniform(&self.raw);
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UniformBuffer<B>
where
  B: Backend,
{
  pub(crate) raw: B::UniformBuffer,
}

impl<B> Drop for UniformBuffer<B>
where
  B: Backend,
{
  fn drop(&mut self) {
    B::drop_uniform_buffer(&self.raw);
  }
}
