pub mod g2d;
pub mod g3d;

mod mesh;
pub use mesh::Mesh;

mod camera;
pub use camera::{Camera, CameraMode};

use luminance_sdl2::GL33Surface;
use luminance_gl::GL33;

use luminance::{Semantics, Vertex, UniformInterface};
use luminance::context::GraphicsContext;
use luminance::shader::{Program, Uniform};
use luminance::tess::Mode;

use glam::{Vec3, Quat};
use crate::Transform;

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
    pub position: VertexPosition,
    pub color: VertexColor,
}

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub offset: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub view: Uniform<[[f32; 4]; 4]>,
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

//TODO: Make this work for all different formats
//      that can be loaded from just a byte array.
/// Incredibly ugly mesh import, only for GLTF format.
pub fn meshes_from_bytes(bytes: Vec<u8>) -> Vec<(Mesh, Transform)> {
    let (document, buffers, images) = gltf::import_slice(bytes.as_slice()).expect("Failed to import bytes as glTF 2.0 data!");
    let mut result = Vec::new();
    let surface = unsafe { &mut crate::ROCK.as_mut().unwrap().surface };
    for mesh in document.meshes() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            for pos in reader.read_positions().expect("No positional data found!") {
                vertices.push(
                    VertexType::new(
                        VertexPosition::new(pos),
                        VertexColor::new([1.0, 1.0, 1.0])
                    )
                );
            }
            for index in reader.read_indices().expect("No index data found!").into_u32() {
                indices.push(index);
            }
        }
        let mesh = Mesh::new(surface, |builder| {
            builder.set_vertices(vertices.clone())
                   .set_indices(indices.clone())
                   .set_mode(Mode::Triangle)
        });
        let pos = Vec3::new(0.0, 0.0, 0.0);
        let rot = Quat::identity();
        let scale = Vec3::new(0.075, 0.075, 0.075);
        let transform = Transform::new(pos, rot, scale);
        result.push((mesh, transform));
    }
    result
}
