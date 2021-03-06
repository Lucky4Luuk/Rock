use luminance::tess::Mode;

use super::{
    VertexType,
    VertexPosition,
    VertexColor,
    VertexUV,
    VertexNormal,
    VertexTangent,

    mesh::Mesh,
};

pub const TRIANGLE: [VertexType; 3] = [
    VertexType::new(
        VertexPosition::new([-1.0, -1.0, 0.0]),
        VertexColor::new([1.0, 0.0, 0.0]),
        VertexUV::new([0.0, 0.0]),
        VertexNormal::new([0.0, 0.0, -1.0]),
        VertexTangent::new([1.0, 0.0, 0.0, 1.0]),
    ),
    VertexType::new(
        VertexPosition::new([1.0, -1.0, 0.0]),
        VertexColor::new([0.0, 1.0, 0.0]),
        VertexUV::new([1.0, 0.0]),
        VertexNormal::new([0.0, 0.0, -1.0]),
        VertexTangent::new([1.0, 0.0, 0.0, 1.0]),
    ),
    VertexType::new(
        VertexPosition::new([0.0, 1.0, 0.0]),
        VertexColor::new([0.0, 0.0, 1.0]),
        VertexUV::new([0.0, 1.0]),
        VertexNormal::new([0.0, 0.0, -1.0]),
        VertexTangent::new([1.0, 0.0, 0.0, 1.0]),
    ),
];

pub fn create_triangle(surface: &mut luminance_sdl2::GL33Surface) -> Mesh {
    Mesh::new(surface, |builder| {
        builder.set_vertices(&TRIANGLE[..])
                .set_indices(Vec::<u32>::new())
                .set_mode(Mode::Triangle)
    })
}
