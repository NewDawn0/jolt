//! Rendering traits and types.

pub mod webgl2;

/// Renderable objects must implement tessellation.
pub trait Renderable {
    fn tesselate(&self, buf: &mut Vec<Vertex>);
}

/// 2D vertex.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
}
