use crate::render::Renderable;

/// A region containing renderable objects.
pub struct Chunk {
    /// Vertex data for GPU upload.
    pub draw_buffer: Vec<f32>,
    /// Objects to render in this chunk.
    pub objects: Vec<Box<dyn Renderable>>,
    /// X and Y bounds of this chunk.
    pub span: [[f32; 2]; 2],
}

impl Chunk {
    /// Creates a new chunk.
    pub fn new(span: [[f32; 2]; 2]) -> Self {
        Self {
            draw_buffer: vec![],
            objects: vec![],
            span,
        }
    }
    /// Gathers tessellated data from all objects.
    pub fn collect(&mut self) {
        self.draw_buffer.clear();
        self.objects
            .iter()
            .for_each(|e| e.tesselate(&mut self.draw_buffer));
    }
}
