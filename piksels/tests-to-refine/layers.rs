// this was taken from piksels-core; needs refinement

use std::collections::HashSet;

use piksels_backend::{
  shader::{ShaderSources, UniformType, UniformTypeBase},
  swap_chain::SwapChainMode,
  texture::{MagFilter, MinFilter, Sampling, Storage, Wrap},
  vertex_array::{MemoryLayout, VertexArrayData},
  Backend,
};
use piksels_core::{device::Device, layers::LayerCommons};

#[test]
fn simple_layers() {
  fn run<B>(device: &Device<B>) -> Result<(), B::Err>
  where
    B: Backend,
  {
    let layers = device.new_layers()?;
    let render_targets =
      device.new_render_targets(HashSet::default(), None, Storage::Flat1D { width: 10 })?;
    let shader = device.new_shader(ShaderSources::default())?;
    let uni_0 = shader.uniform("uni_0", UniformTypeBase::Float)?;

    // vertex arrays
    let foo = device.new_vertex_array(
      VertexArrayData::new(Vec::new(), MemoryLayout::Interleaved { data: Vec::new() }),
      VertexArrayData::new(Vec::new(), MemoryLayout::Interleaved { data: Vec::new() }),
      [],
    )?;
    let bar = device.new_vertex_array(
      VertexArrayData::new(Vec::new(), MemoryLayout::Interleaved { data: Vec::new() }),
      VertexArrayData::new(Vec::new(), MemoryLayout::Interleaved { data: Vec::new() }),
      [],
    )?;

    // textures
    let texture_foo = device.new_texture(
      Storage::Flat1D { width: 10 },
      Sampling {
        wrap_r: Wrap::ClampToEdge,
        wrap_s: Wrap::ClampToEdge,
        wrap_t: Wrap::ClampToEdge,
        min_filter: MinFilter::Nearest,
        mag_filter: MagFilter::Nearest,
        depth_comparison: None,
      },
    )?;
    let texture_bar = device.new_texture(
      Storage::Flat1D { width: 10 },
      Sampling {
        wrap_r: Wrap::ClampToEdge,
        wrap_s: Wrap::ClampToEdge,
        wrap_t: Wrap::ClampToEdge,
        min_filter: MinFilter::Nearest,
        mag_filter: MagFilter::Nearest,
        depth_comparison: None,
      },
    )?;

    layers
      // layers
      .render_targets(&render_targets)?
        .shader(&shader)?
          .draw(&foo)?
          .draw(&bar)?
          .done()
        .done()
      .done()?;

    Ok(())
  }
}
