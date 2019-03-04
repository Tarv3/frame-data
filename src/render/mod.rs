pub mod shape_gen;
pub mod render_windows;
pub mod image_render;

use glium::*;
use na::geometry::*;
use na::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
}

implement_vertex!(Vertex, pos);

pub struct BoxRenderer {
    program: Program,
    box_buffer: VertexBuffer<Vertex>,

}

impl BoxRenderer {
    pub fn render_box<S: Surface + ?Sized>(&self, target: &mut S, box_dims: [f32; 2], [x, y]: [f32; 2], angle: f32) -> Result<(), Box<dyn std::error::Error>> {
        let translation = Translation::from(Vector2::new(x, y));
        let rotation = UnitComplex::new(angle);

        let uniforms = uniform!(dims: box_dims);

        Ok(())
    }
}

