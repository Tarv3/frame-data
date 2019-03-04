use glium::{
    VertexBuffer,
    backend::glutin::Display,
};
use super::Vertex;

pub fn gen_box(display: &Display) -> VertexBuffer<Vertex> {
    let data = [
        Vertex { pos: [-0.5, -0.5 ]},
        Vertex { pos: [ 0.5, -0.5 ]},
        Vertex { pos: [-0.5, 0.5 ]},
        Vertex { pos: [ 0.5, 0.5 ]},
    ];

    VertexBuffer::new(display, &data).unwrap()
}
