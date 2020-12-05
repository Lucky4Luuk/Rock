use luminance::tess::{Tess, Mode};
use luminance_gl::GL33;

use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::graphics::Mesh;
use crate::graphics::g2d::TRIANGLE;
use crate::graphics::VertexType;

/// Wrapper around the many types of meshes, to provide a single
/// interface for Lua.
#[derive(Clone)]
pub struct LuaMesh {
    mesh: Mesh,
}

impl LuaMesh {
    pub fn new_2d() -> Self {
        let mesh = unsafe { Mesh::new(&mut crate::ROCK.as_mut().unwrap().surface, |builder| {
            builder.set_vertices(&TRIANGLE[..])
                .set_mode(Mode::Triangle)
        }) };
        Self {
            mesh: mesh,
        }
    }

    pub fn tess(&self) -> &Tess<GL33, VertexType> {
        self.mesh.tess()
    }
}

impl UserData for LuaMesh {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            Ok("RockMesh") //TODO: Actual information about the object
        });
    }
}

pub fn mesh_constructor() -> LuaMesh {
    LuaMesh::new_2d()
}
