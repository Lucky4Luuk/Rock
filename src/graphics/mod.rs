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
    #[sem(name = "uv", repr = "[f32; 2]", wrapper = "VertexUV")]
    UV,
    #[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    #[sem(name = "tangent", repr = "[f32; 4]", wrapper = "VertexTangent")]
    Tangent,
}

#[derive(Copy, Clone, Debug, PartialEq, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct VertexType {
    pub position: VertexPosition,
    pub color: VertexColor,
    pub uv: VertexUV,
    pub normal: VertexNormal,
    pub tangent: VertexTangent,
}

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub offset: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub view: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub normal_matrix: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub cam_pos: Uniform<[f32; 3]>,
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

//TODO: Separate GLB and GLTF because there might be differences
//      between the two when it comes to loading.
#[non_exhaustive]
pub enum MeshByteFormat {
    GLB,
    GLTF,
    Other,
}

impl MeshByteFormat {
    pub fn from_string(format: String) -> Self {
        match format.to_lowercase().as_str() {
            "gltf" => Self::GLTF,
            "glb" => Self::GLB,

            _ => Self::Other,
        }
    }
}

pub fn meshes_from_bytes(bytes: Vec<u8>, format: MeshByteFormat) -> Vec<(Mesh, Transform)> {
    match format {
        GLB => gltf_meshes_from_bytes(bytes),
        _ => unimplemented!(),
    }
}

/// Incredibly ugly gltf mesh import
fn gltf_meshes_from_bytes(bytes: Vec<u8>) -> Vec<(Mesh, Transform)> {
    let (document, buffers, images) = gltf::import_slice(bytes.as_slice()).expect("Failed to import bytes as glTF 2.0 data!");
    let mut result = Vec::new();
    let surface = unsafe { &mut crate::ROCK.as_mut().unwrap().surface };
    for mesh in document.meshes() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let pos_vec: Vec<[f32; 3]> = reader.read_positions().expect("No positional data found!").collect();
            let rgb_vec: Option<Vec<[f32; 3]>> = match reader.read_colors(0) {
                Some(data) => Some(data.into_rgb_f32().collect()),
                None => None,
            };
            let uv_vec: Option<Vec<[f32; 2]>> = match reader.read_tex_coords(0) {
                Some(data) => Some(data.into_f32().collect()),
                None => None,
            };
            let normal_vec: Vec<[f32; 3]> = reader.read_normals().expect("No normal data found!").collect();
            let tangent_vec: Vec<[f32; 4]> = reader.read_tangents().expect("No tangent data found!").collect();
            //TODO: Check array bounds
            for i in 0..pos_vec.len() {
                let pos = pos_vec[i];
                let rgb = match rgb_vec {
                    Some(ref val) => val[i],
                    None => [1.0, 1.0, 1.0],
                };
                let uv = match uv_vec {
                    Some(ref val) => val[i],
                    None => [0.0, 0.0],
                };
                let normal = normal_vec[i];
                let tangent = tangent_vec[i];
                vertices.push(
                    VertexType::new(
                        VertexPosition::new(pos),
                        VertexColor::new(rgb),
                        VertexUV::new(uv),
                        VertexNormal::new(normal),
                        VertexTangent::new(tangent),
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
        let scale = Vec3::new(1.0, 1.0, 1.0); //0.075, 0.075, 0.075
        let transform = Transform::new(pos, rot, scale);
        result.push((mesh, transform));
    }
    result
}
