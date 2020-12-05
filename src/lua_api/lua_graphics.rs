use luminance::context::GraphicsContext as _;

use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

pub mod lua_mesh;

use super::LuaApi;

use lua_mesh::{LuaMesh, mesh_constructor};

///Loads rock.graphics
pub fn load_graphics_table(lua: &LuaApi) -> Result<()> {
    let graphics_table = lua.create_table()?;

    let clear_func = lua.create_function(|_,(r,g,b,a)| {
        clear(r,g,b,a);
        Ok(())
    })?;
    graphics_table.set("clear", clear_func)?;
    let mesh_func = lua.create_function(|_,()| {
        let mesh = mesh_constructor();
        Ok(mesh)
    })?;
    graphics_table.set("mesh", mesh_func)?;
    let draw_func = lua.create_function(|_,(mesh)| {
        draw(mesh);
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

fn draw(mesh: LuaMesh) {
    use crate::ROCK;
    use luminance::render_state::RenderState;

    let back_buffer = unsafe { ROCK.as_mut().unwrap().surface.back_buffer().expect("Failed to get backbuffer!") };

    let render = unsafe { ROCK.as_mut().unwrap().surface.new_pipeline_gate().pipeline(
        &back_buffer,
        &ROCK.as_mut().unwrap().pipeline_state,
        |_pipeline, mut shd_gate| {
            shd_gate.shade(&mut ROCK.as_mut().unwrap().default_program, |_, _, mut rdr_gate| {
                rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                    tess_gate.render(mesh.tess())
                })
            })
        },
    ).assume() };

    if !render.is_ok() {
        panic!("Renderer ran into unknown error!");
    }
}
