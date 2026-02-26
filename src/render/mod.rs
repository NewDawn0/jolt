//! Rendering traits and types.

pub mod webgl2;

/// Renderable objects must implement tessellation.
pub trait Renderable {
    fn tesselate(&self, buf: &mut Vec<f32>);
}
