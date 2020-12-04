use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

///Loads rock.graphics
pub fn load_graphics_table<'a>(lua: &'a Lua) -> Result<()> {
    let graphics_table = lua.create_table()?;
    let clear_func = lua.create_function(|_,(r,g,b,a)| {
        let mut state = crate::pipeline_state.lock().expect("Unable to acquire lock!");
        (*state).clear_color = [r,g,b,a];
        Ok(())
    })?;
    graphics_table.set("clear", clear_func)?;
    let globals = lua.globals();
    let rock_table: Table = globals.get("rock")?;
    rock_table.set("graphics", graphics_table)?;
    Ok(())
}
