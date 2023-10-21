use std::{
  collections::HashSet,
  sync::{Arc, Mutex},
};

use piksels_backend::{
  render_targets::{ColorAttachmentPoint, DepthStencilAttachmentPoint},
  shader::ShaderSources,
  swap_chain::SwapChainMode,
  texture::{Sampling, Storage},
  vertex_array::VertexArrayData,
  Backend, BackendInfo,
};

use crate::{
  cache::ScarceCache, cmd_buf::CmdBuf, render_targets::RenderTargets, shader::Shader,
  swap_chain::SwapChain, texture::Texture, vertex_array::VertexArray,
};

#[derive(Debug)]
pub struct Device<B>
where
  B: Backend,
{
  backend: B,
  cache: Arc<Mutex<ScarceCache<B>>>,
}

macro_rules! cache_options {
  ($($var:ident : $ty:ty),* $(,)?) => {
    $(
      pub fn $var(&self) -> Result<$ty, B::Err> {
        let mut cache = self.cache.lock().map_err(|e| e.into())?;

        match cache.$var() {
          Some(x) => Ok(x.clone()),

          None => {
            let x = self.backend.$var()?;
            *cache.$var() = Some(x.clone());
            Ok(x)
          }
        }
      }
    )*
  };
}

impl<B> Device<B>
where
  B: Backend,
{
  cache_options!(
    author: String,
    name: String,
    version: String,
    shading_lang_version: String,
    info: BackendInfo,
  );

  pub fn new(backend: B) -> Result<Self, B::Err> {
    let cache = Arc::new(Mutex::new(ScarceCache::new(&backend)?));
    Ok(Self { backend, cache })
  }

  pub fn new_vertex_array(
    &self,
    vertices: VertexArrayData,
    instances: VertexArrayData,
    indices: impl Into<Vec<u32>>,
  ) -> Result<VertexArray<B>, B::Err> {
    let indices = indices.into();

    let vertex_array = self
      .backend
      .new_vertex_array(&vertices, &instances, &indices)
      .map(|raw| {
        VertexArray::from_raw(
          raw,
          Arc::downgrade(&self.cache),
          vertices,
          instances,
          indices,
        )
      })?;

    self
      .cache
      .lock()
      .map_err(From::from)?
      .track_vertex_array(&vertex_array.raw);

    Ok(vertex_array)
  }

  pub fn new_render_targets(
    &self,
    color_attachment_points: HashSet<ColorAttachmentPoint>,
    depth_stencil_attachment_point: Option<DepthStencilAttachmentPoint>,
    storage: Storage,
  ) -> Result<RenderTargets<B>, B::Err> {
    let render_targets = self
      .backend
      .new_render_targets(
        color_attachment_points,
        depth_stencil_attachment_point,
        storage,
      )
      .map(|raw| RenderTargets::from_raw(raw, Arc::downgrade(&self.cache)))?;

    self
      .cache
      .lock()
      .map_err(From::from)?
      .track_render_targets(&render_targets.raw);

    Ok(render_targets)
  }

  pub fn new_shader(&self, sources: ShaderSources) -> Result<Shader<B>, B::Err> {
    let shader = self
      .backend
      .new_shader(sources)
      .map(|raw| Shader::from_raw(raw, Arc::downgrade(&self.cache)))?;

    self
      .cache
      .lock()
      .map_err(From::from)?
      .track_shader(&shader.raw);

    Ok(shader)
  }

  pub fn new_texture(&self, storage: Storage, sampling: Sampling) -> Result<Texture<B>, B::Err> {
    let texture = self
      .backend
      .new_texture(storage, sampling)
      .map(|raw| Texture::from_raw(raw, Arc::downgrade(&self.cache)))?;

    self
      .cache
      .lock()
      .map_err(From::from)?
      .track_texture(&texture.raw);

    Ok(texture)
  }

  pub fn new_cmd_buf(&self) -> Result<CmdBuf<B>, B::Err> {
    let cmd_buf = self
      .backend
      .new_cmd_buf()
      .map(|cmd_buf| CmdBuf::from_raw(cmd_buf, Arc::downgrade(&self.cache)))?;

    self
      .cache
      .lock()
      .map_err(From::from)?
      .track_cmd_buf(&cmd_buf.raw);

    Ok(cmd_buf)
  }

  pub fn new_swap_chain(
    &self,
    width: u32,
    height: u32,
    mode: SwapChainMode,
  ) -> Result<SwapChain<B>, B::Err> {
    let swap_chain = self
      .backend
      .new_swap_chain(width, height, mode)
      .map(|raw| SwapChain::from_raw(raw, Arc::downgrade(&self.cache)))?;

    self
      .cache
      .lock()
      .map_err(From::from)?
      .track_swap_chain(&swap_chain.raw);

    Ok(swap_chain)
  }
}
