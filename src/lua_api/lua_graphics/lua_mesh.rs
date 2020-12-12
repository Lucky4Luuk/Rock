use luminance::tess::{Tess, Mode};
use luminance_gl::GL33;

use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::graphics::Mesh;
use crate::graphics::g2d::TRIANGLE;
use crate::graphics::{VertexType, VertexPosition, VertexColor};
use crate::lua_api::lua_math::LuaVec3;

/// Wrapper around the many types of meshes, to provide a single
/// interface for Lua.
#[derive(Clone)]
pub struct LuaMesh {
    mesh: Mesh,
}

impl LuaMesh {
    pub fn new(vertices: &[VertexType]) -> Self {
        let mesh = unsafe { Mesh::new(&mut crate::ROCK.as_mut().unwrap().surface, |builder| {
            builder.set_vertices(vertices)
                   .set_indices(Vec::<u32>::new())
                   .set_mode(Mode::Triangle)
        }) };
        Self {
            mesh: mesh,
        }
    }

    pub fn from_mesh(mesh: Mesh) -> Self {
        Self {
            mesh: mesh,
        }
    }

    pub fn tess(&self) -> &Tess<GL33, VertexType, u32> {
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

pub fn mesh_constructor(lua_verts: Table) -> Result<LuaMesh> {
    let mut vertices = Vec::new();
    for i in 0..lua_verts.len()? {
        let lua_vert: Table = lua_verts.get(i + 1)?;
        let pos: LuaVec3 = lua_vert.get(1)?;
        let rgb: LuaVec3 = lua_vert.get(2)?;
        let vert = VertexType::new(
            VertexPosition::new((*pos.vec).into()),
            VertexColor::new((*rgb.vec).into()),
        );
        vertices.push(vert);
    }
    Ok(LuaMesh::new(&vertices))
}
