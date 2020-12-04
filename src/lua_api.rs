use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

pub mod lua_graphics;

/// Helper function that binds some custom functions
pub fn init_lua() -> Lua {
    let lua = Lua::new();
    {
        let globals = lua.globals();
        let custom_print = lua.create_function(|_, list: Variadic<String>| {
            lua_print(list.iter().collect());
            Ok(())
        }).expect("Failed to create `print` function!");
        globals.set("print", custom_print).expect("Failed to overwrite `print` function!");
    }
    load_main_table(&lua).expect("Failed to load `rock` table!");
    lua_graphics::load_graphics_table(&lua).expect("Failed to load `main` table!");
    lua
}

pub fn load_code<'a>(lua: &'a Lua, code: &'a str) -> Chunk<'a, 'a> {
    lua.load(code)
}

pub fn call_rock_func<'a, A: ToLua<'a>>(lua: &'a Lua, func_name: &'a str, args: A) -> Result<()> {
    {
        let globals = lua.globals();
        let rock_table: Table = globals.get("rock")?;
        let func: Function = rock_table.get(func_name)?;
        func.call::<_, ()>(args)?;
    }
    Ok(())
}

fn load_main_table<'a>(lua: &'a Lua) -> Result<()> {
    let rock_table = lua.create_table()?;
    let load_func = lua.create_function(|_,()| {
        Ok(())
    })?;
    rock_table.set("load", load_func)?;
    let update_func = lua.create_function(|_,dt: f32| {
        Ok(())
    })?;
    rock_table.set("update", update_func)?;
    let draw_func = lua.create_function(|_,()| {
        Ok(())
    })?;
    rock_table.set("draw", draw_func)?;
    let globals = lua.globals();
    globals.set("rock", rock_table)?;
    Ok(())
}

fn lua_print(data: Vec<&String>) {
    let mut output = String::new();
    for s in data {
        output.push_str(s);
    }
    info!(target: "lua", "{}", output);
}
