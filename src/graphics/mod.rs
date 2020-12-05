pub mod g2d;
pub mod g3d;

mod mesh;
pub use mesh::Mesh;

use luminance_sdl2::GL33Surface;
use luminance_gl::GL33;

use luminance::{Semantics, Vertex};
use luminance::context::GraphicsContext;
use luminance::shader::Program;

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

const VS_STR: &str = include_str!("vs2d.glsl");
const FS_STR: &str = include_str!("fs2d.glsl");

pub type ShaderProgram = Program<GL33, VertexSemantics, (), ()>;
pub fn get_default_program(surface: &mut GL33Surface) -> ShaderProgram {
    surface.new_shader_program::<VertexSemantics, (), ()>()
           .from_strings(VS_STR, None, None, FS_STR)
           .expect("Failed to compile shaders!")
           .ignore_warnings()
}
