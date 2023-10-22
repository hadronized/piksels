use piksels_backend::{shader::UniformType, Backend};

#[derive(Debug)]
pub struct Shader<B>
where
  B: Backend,
{
  pub(crate) raw: B::Shader,
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

  pub fn texture_binding_point(
    &self,
    name: impl AsRef<str>,
  ) -> Result<TextureBindingPoint<B>, B::Err> {
    B::get_texture_binding_point(&self.raw, name.as_ref()).map(|raw| TextureBindingPoint { raw })
  }

  pub fn uniform_buffer_binding_point(
    &self,
    name: impl AsRef<str>,
  ) -> Result<UniformBufferBindingPoint<B>, B::Err> {
    B::get_uniform_buffer_binding_point(&self.raw, name.as_ref())
      .map(|raw| UniformBufferBindingPoint { raw })
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Uniform<B>
where
  B: Backend,
{
  pub(crate) raw: B::Uniform,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UniformBuffer<B>
where
  B: Backend,
{
  pub(crate) raw: B::UniformBuffer,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UniformBufferBindingPoint<B>
where
  B: Backend,
{
  pub(crate) raw: B::UniformBufferBindingPoint,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TextureBindingPoint<B>
where
  B: Backend,
{
  pub(crate) raw: B::TextureBindingPoint,
}
