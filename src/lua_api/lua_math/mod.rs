use mlua::{UserData, MetaMethod, Result, UserDataMethods, Table};
use glam::*;

use crate::math::Transform;
use super::LuaApi;

///Loads rock.math
pub fn load_math_table(lua: &LuaApi) -> Result<()> {
    let math_table = lua.create_table()?;

    let vec3_func = lua.create_function(|_,(x,y,z)| {
        Ok(vec3_constructor(x,y,z))
    })?;
    math_table.set("vec3", vec3_func)?;
    let quat_euler_func = lua.create_function(|_,(yaw, pitch, roll)| {
        Ok(quat_euler_constructor(yaw, pitch, roll))
    })?;
    math_table.set("quat_euler", quat_euler_func)?;
    let transform_func = lua.create_function(|_,(pos, rot, scale)| {
        Ok(transform_constructor(pos, rot, scale))
    })?;
    math_table.set("transform", transform_func)?;

    let globals = lua.globals();
    let rock_table: Table = globals.get("rock")?;
    rock_table.set("math", math_table)?;
    Ok(())
}

#[derive(Copy, Clone)]
pub struct LuaVec3 {
    pub vec: Vec3,
}

impl UserData for LuaVec3 {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            Ok(format!("Vector {{ vec: {} }}", obj.vec))
        });

        methods.add_meta_function(MetaMethod::Eq, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(a.vec == b.vec)
        });

        methods.add_meta_function(MetaMethod::Lt, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(a.vec < b.vec)
        });

        methods.add_meta_function(MetaMethod::Le, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(a.vec <= b.vec)
        });

        methods.add_meta_function(MetaMethod::Add, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: a.vec + b.vec })
        });

        methods.add_meta_function(MetaMethod::Sub, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: a.vec - b.vec })
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: a.vec * b.vec })
        });

        methods.add_meta_function(MetaMethod::Div, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: a.vec / b.vec })
        });

        methods.add_meta_function(MetaMethod::Pow, |_, (a,b): (LuaVec3, f32)| {
            Ok(LuaVec3 { vec: a.vec.powf(b) })
        });

        methods.add_meta_function(MetaMethod::Unm, |_, a: LuaVec3| {
            Ok(LuaVec3 { vec: -a.vec })
        });
    }
}

pub fn vec3_constructor(x: f32, y: f32, z: f32) -> LuaVec3 {
    LuaVec3 {
        vec: Vec3::new(x,y,z)
    }
}

#[derive(Copy, Clone)]
pub struct LuaQuat {
    pub quat: Quat,
}

impl UserData for LuaQuat {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            Ok(format!("Transform {{ quat: {} }}", obj.quat))
        });

        methods.add_meta_function(MetaMethod::Eq, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(a.quat == b.quat)
        });

        methods.add_meta_function(MetaMethod::Lt, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(a.quat < b.quat)
        });

        methods.add_meta_function(MetaMethod::Le, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(a.quat <= b.quat)
        });

        methods.add_meta_function(MetaMethod::Add, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(LuaQuat { quat: a.quat + b.quat })
        });

        methods.add_meta_function(MetaMethod::Sub, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(LuaQuat { quat: a.quat - b.quat })
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(LuaQuat { quat: a.quat * b.quat })
        });

        methods.add_meta_function(MetaMethod::Div, |_, (a,b): (LuaQuat, f32)| {
            Ok(LuaQuat { quat: a.quat / b })
        });

        methods.add_meta_function(MetaMethod::Unm, |_, a: LuaQuat| {
            Ok(LuaQuat { quat: -a.quat })
        });
    }
}

pub fn quat_euler_constructor(yaw: f32, pitch: f32, roll: f32) -> LuaQuat {
    LuaQuat {
        quat: Quat::from_rotation_ypr(yaw, pitch, roll)
    }
}

#[derive(Copy, Clone)]
pub struct LuaTransform {
    pub transform: Transform,
}

impl UserData for LuaTransform {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            let pos = obj.transform.pos();
            let rot = obj.transform.rot();
            let scale = obj.transform.scale();
            Ok(format!("Transform {{ pos: {} - rot: {} - scale: {} }}", pos, rot, scale))
        });
    }
}

pub fn transform_constructor(pos: LuaVec3, rot: LuaQuat, scale: LuaVec3) -> LuaTransform {
    let transform = Transform::new(pos.vec, rot.quat, scale.vec);
    LuaTransform {
        transform: transform
    }
}
