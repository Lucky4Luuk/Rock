pub mod g2d;
pub mod g3d;

mod mesh;
pub use mesh::Mesh;

use luminance_sdl2::GL33Surface;
use luminance_gl::GL33;

use luminance::{Semantics, Vertex, UniformInterface};
use luminance::context::GraphicsContext;
use luminance::shader::{Program, Uniform};

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

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    // #[uniform(unbound)]
    // projection: Uniform<[[f32; 4]; 4]>,
    // #[uniform(unbound)]
    // view: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub offset: Uniform<[[f32; 4]; 4]>,
}

const VS_STR: &str = include_str!("vs2d.glsl");
const FS_STR: &str = include_str!("fs2d.glsl");

pub type ShaderProgram = Program<GL33, VertexSemantics, (), ShaderInterface>;
pub fn get_default_program(surface: &mut GL33Surface) -> ShaderProgram {
    surface.new_shader_program::<VertexSemantics, (), ShaderInterface>()
           .from_strings(VS_STR, None, None, FS_STR)
           .expect("Failed to compile shaders!")
           .ignore_warnings()
}
