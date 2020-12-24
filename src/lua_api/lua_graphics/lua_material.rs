use mlua::{Chunk, Function, Table, Lua, prelude::ToLua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

/// A lua material, with no rust counterpart.
/// All variables for the material are uploaded using
/// `shader:send()`, including materials
#[derive(Copy, Clone)]
pub struct LuaMaterial;

impl UserData for LuaMaterial {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, _: Self| {
            Ok("Material")
        });
    }
}
