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
  ) -> Result<ShaderTextureBindingPoint<B>, B::Err> {
    B::get_shader_texture_binding_point(&self.raw, name.as_ref())
      .map(|raw| ShaderTextureBindingPoint { raw })
  }

  pub fn uniform_buffer_binding_point(
    &self,
    name: impl AsRef<str>,
  ) -> Result<ShaderUniformBufferBindingPoint<B>, B::Err> {
    B::get_shader_uniform_buffer_binding_point(&self.raw, name.as_ref())
      .map(|raw| ShaderUniformBufferBindingPoint { raw })
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Uniform<B>
where
  B: Backend,
{
  pub(crate) raw: B::Uniform,
}

#[derive(Debug)]
pub struct UniformBuffer<B>
where
  B: Backend,
{
  pub(crate) raw: B::UniformBuffer,
}

#[derive(Debug)]
pub struct UniformBufferBindingPoint<B>
where
  B: Backend,
{
  pub(crate) raw: B::UniformBufferBindingPoint,
}

impl<B> UniformBufferBindingPoint<B>
where
  B: Backend,
{
  pub(crate) fn from_raw(raw: B::UniformBufferBindingPoint) -> Self {
    Self { raw }
  }
}

#[derive(Debug)]
pub struct ShaderUniformBufferBindingPoint<B>
where
  B: Backend,
{
  pub(crate) raw: B::ShaderUniformBufferBindingPoint,
}

#[derive(Debug)]
pub struct ShaderTextureBindingPoint<B>
where
  B: Backend,
{
  pub(crate) raw: B::ShaderTextureBindingPoint,
}
