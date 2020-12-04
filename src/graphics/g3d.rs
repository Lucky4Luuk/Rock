use luminance::{Semantics, Vertex};

#[derive(Copy, Clone, Debug, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[f32; 3]", wrapper = "VertexColor")]
    Color,
}

#[derive(Copy, Clone, Debug, PartialEq, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct VertexType {
    position: VertexPosition,
    color: VertexColor,
}
