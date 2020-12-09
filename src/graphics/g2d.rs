use luminance::tess::Mode;

use super::{
    VertexType,
    VertexPosition,
    VertexColor,
    mesh::Mesh,
};

pub const TRIANGLE: [VertexType; 3] = [
    VertexType::new(
        VertexPosition::new([-1.0, -1.0, 0.0]),
        VertexColor::new([1.0, 0.0, 0.0]),
    ),
    VertexType::new(
        VertexPosition::new([1.0, -1.0, 0.0]),
        VertexColor::new([0.0, 1.0, 0.0]),
    ),
    VertexType::new(
        VertexPosition::new([0.0, 1.0, 0.0]),
        VertexColor::new([0.0, 0.0, 1.0])
    ),
];

pub fn create_triangle(surface: &mut luminance_sdl2::GL33Surface) -> Mesh {
    Mesh::new(surface, |builder| {
        builder.set_vertices(&TRIANGLE[..])
                .set_mode(Mode::Triangle)
    })
}
