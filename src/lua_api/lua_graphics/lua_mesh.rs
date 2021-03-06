use luminance::tess::{Tess, Mode};
use luminance_gl::GL33;

use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::graphics::Mesh;
use crate::graphics::g2d::TRIANGLE;
use crate::graphics::{VertexType, VertexPosition, VertexColor, VertexUV, VertexNormal, VertexTangent};
use crate::lua_api::lua_math::{LuaVec2, LuaVec3, LuaVec4};

/// Wrapper around the many types of meshes, to provide a single
/// interface for Lua.
#[derive(Clone)]
pub struct LuaMesh {
    pub mesh: Mesh,
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
        let rgb: LuaVec3 = lua_vert.get(4)?;
        let uv: LuaVec2 = lua_vert.get(5)?;
        let normal: LuaVec3 = lua_vert.get(2)?;
        let tangent: LuaVec4 = lua_vert.get(3)?;
        let vert = VertexType::new(
            VertexPosition::new((*pos.vec).into()),
            VertexColor::new((*rgb.vec).into()),
            VertexUV::new((*uv.vec).into()),
            VertexNormal::new((*normal.vec).into()),
            VertexTangent::new((*tangent.vec).into()),
        );
        vertices.push(vert);
    }
    Ok(LuaMesh::new(&vertices))
}
