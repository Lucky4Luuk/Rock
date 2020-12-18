use std::sync::Arc;
use luminance::context::GraphicsContext as _;

use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

pub mod lua_mesh;

use super::LuaApi;

use lua_mesh::{LuaMesh, mesh_constructor};
use super::lua_math::{LuaTransform, LuaVec2,LuaVec3};

///Loads rock.graphics
pub fn load_graphics_table(lua: &LuaApi) -> Result<()> {
    let graphics_table = lua.create_table()?;

    let clear_func = lua.create_function(|_,(r,g,b,a)| {
        clear(r,g,b,a);
        Ok(())
    })?;
    graphics_table.set("clear", clear_func)?;
    let mesh_func = lua.create_function(|_,vertices| {
        mesh_constructor(vertices)
    })?;
    graphics_table.set("mesh", mesh_func)?;
    let load_mesh_func = lua.create_function(|_,(path, format): (String, String)| {
        use crate::graphics::MeshByteFormat;
        let bformat = MeshByteFormat::from_string(format);
        // let vertices = crate::graphics::g2d::TRIANGLE;
        let mut bytes = Vec::new();
        unsafe { crate::ROCK.as_ref().unwrap().vfs.read_bytes(&path, &mut bytes).expect("Failed to load file!"); }
        let mesh_vec = crate::graphics::meshes_from_bytes(bytes, bformat);
        let mut meshes = Vec::new();
        let mut transforms = Vec::new();
        for (mesh, transform) in mesh_vec {
            let lua_mesh = LuaMesh::from_mesh(mesh);
            let lua_transform = LuaTransform::from_transform(transform);
            meshes.push(lua_mesh);
            transforms.push(lua_transform)
        }
        Ok((meshes, transforms))
    })?;
    graphics_table.set("load_mesh", load_mesh_func)?;
    let draw_func = lua.create_function(|_,(mesh, transform)| {
        draw(mesh, transform);
        Ok(())
    })?;
    graphics_table.set("draw", draw_func)?;

    let globals = lua.globals();
    let rock_table: Table = globals.get("rock")?;
    rock_table.set("graphics", graphics_table)?;
    Ok(())
}

fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        if let Some(ref mut rock) = crate::ROCK {
            rock.pipeline_state = rock.pipeline_state.clone().set_clear_color([r,g,b,a]);
        } else {
            panic!("Failed to get `ROCK`!");
        }
    }
}

fn draw(mesh: LuaMesh, transform: LuaTransform) {
    use crate::ROCK;
    use luminance::render_state::RenderState;

    let back_buffer = unsafe { ROCK.as_mut().unwrap().surface.back_buffer().expect("Failed to get backbuffer!") };
    let camera = unsafe { &ROCK.as_ref().unwrap().camera };

    let render = unsafe { ROCK.as_mut().unwrap().surface.new_pipeline_gate().pipeline(
        &back_buffer,
        &ROCK.as_mut().unwrap().get_render_state(),
        |_pipeline, mut shd_gate| {
            shd_gate.shade(&mut ROCK.as_mut().unwrap().default_program, |mut iface, uni, mut rdr_gate| {
                iface.set(&uni.offset, transform.transform.get_matrix().to_cols_array_2d());

                //MVP
                iface.set(&uni.projection, camera.get_proj().to_cols_array_2d());
                iface.set(&uni.view, camera.get_view().to_cols_array_2d());
                iface.set(&uni.normal_matrix, transform.transform.get_normal_matrix().to_cols_array_2d());
                iface.set(&uni.cam_pos, camera.transform.pos.into());

                rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                    tess_gate.render(mesh.tess())
                })
            })
        },
    ).assume() };

    if !render.is_ok() {
        panic!("Renderer ran into unknown error!");
    }

    unsafe {
        // println!("{}", mesh.mesh.tri_count());
        let r = ROCK.as_mut().unwrap();
        r.tri_count += mesh.mesh.tri_count();
        r.draw_calls += 1;
    }
}
