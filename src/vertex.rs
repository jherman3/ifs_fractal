#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub hue: f32,
}

implement_vertex!(Vertex, position, hue);
