pub trait Renderable {
    fn tesselate(&self, buf: &mut Vec<Vertex>);
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
}
