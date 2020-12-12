use std::sync::Arc;
use std::convert::AsMut;

use mlua::{UserData, MetaMethod, Result, UserDataMethods, Table, Value, ToLua};
use glam::*;

use crate::math::Transform;
use super::LuaApi;

///Loads rock.math
pub fn load_math_table(lua: &LuaApi) -> Result<()> {
    let math_table = lua.create_table()?;

    let vec2_func = lua.create_function(|_,(x,y)| {
        Ok(vec2_constructor(x,y))
    })?;
    math_table.set("vec2", vec2_func)?;
    let vec3_func = lua.create_function(|_,(x,y,z)| {
        Ok(vec3_constructor(x,y,z))
    })?;
    math_table.set("vec3", vec3_func)?;
    let quat_euler_func = lua.create_function(|_,(yaw,pitch,roll)| {
        let quat = quat_euler_constructor(yaw, pitch, roll);
        Ok(quat)
    })?;
    math_table.set("quat_euler", quat_euler_func)?;
    let transform_func = lua.create_function(|_,(pos,rot,scale)| {
        Ok(transform_constructor(pos, rot, scale))
    })?;
    math_table.set("transform", transform_func)?;

    let globals = lua.globals();
    let rock_table: Table = globals.get("rock")?;
    rock_table.set("math", math_table)?;
    Ok(())
}

#[derive(Clone)]
pub struct LuaVec2 {
    pub vec: Arc<Vec2>,
}

impl UserData for LuaVec2 {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            Ok(format!("Vector {{ vec: {} }}", obj.vec))
        });

        // Comparisons
        methods.add_meta_function(MetaMethod::Eq, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(*a.vec == *b.vec)
        });

        methods.add_meta_function(MetaMethod::Lt, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(*a.vec < *b.vec)
        });

        methods.add_meta_function(MetaMethod::Le, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(*a.vec <= *b.vec)
        });

        // Math functions
        methods.add_meta_function(MetaMethod::Add, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(LuaVec2 { vec: Arc::new(*a.vec + *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Sub, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(LuaVec2 { vec: Arc::new(*a.vec - *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(LuaVec2 { vec: Arc::new(*a.vec * *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Div, |_, (a,b): (LuaVec2, LuaVec2)| {
            Ok(LuaVec2 { vec: Arc::new(*a.vec / *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Pow, |_, (a,b): (LuaVec2, f32)| {
            Ok(LuaVec2 { vec: Arc::new(a.vec.powf(b)) })
        });

        methods.add_meta_function(MetaMethod::Unm, |_, a: LuaVec2| {
            Ok(LuaVec2 { vec: Arc::new(-*a.vec) })
        });

        // Data related functions
        methods.add_method("getX", |_, obj, ()| {
            Ok(obj.vec.x)
        });

        methods.add_method("getY", |_, obj, ()| {
            Ok(obj.vec.y)
        });

        methods.add_method_mut("setX", |_, obj, x: f32| {
            Arc::make_mut(&mut obj.vec).x = x;
            Ok(())
        });

        methods.add_method_mut("setY", |_, obj, y: f32| {
            Arc::make_mut(&mut obj.vec).y = y;
            Ok(())
        });
    }
}

pub fn vec2_constructor(x: f32, y: f32) -> LuaVec2 {
    LuaVec2 {
        vec: Arc::new(Vec2::new(x,y))
    }
}

#[derive(Clone)]
pub struct LuaVec3 {
    pub vec: Arc<Vec3>,
}

impl UserData for LuaVec3 {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            Ok(format!("Vector {{ vec: {} }}", obj.vec))
        });

        // Comparisons
        methods.add_meta_function(MetaMethod::Eq, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(*a.vec == *b.vec)
        });

        methods.add_meta_function(MetaMethod::Lt, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(*a.vec < *b.vec)
        });

        methods.add_meta_function(MetaMethod::Le, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(*a.vec <= *b.vec)
        });

        // Math functions
        methods.add_meta_function(MetaMethod::Add, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: Arc::new(*a.vec + *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Sub, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: Arc::new(*a.vec - *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: Arc::new(*a.vec * *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Div, |_, (a,b): (LuaVec3, LuaVec3)| {
            Ok(LuaVec3 { vec: Arc::new(*a.vec / *b.vec) })
        });

        methods.add_meta_function(MetaMethod::Pow, |_, (a,b): (LuaVec3, f32)| {
            Ok(LuaVec3 { vec: Arc::new(a.vec.powf(b)) })
        });

        methods.add_meta_function(MetaMethod::Unm, |_, a: LuaVec3| {
            Ok(LuaVec3 { vec: Arc::new(-*a.vec) })
        });

        // Data related functions
        methods.add_method("getX", |_, obj, ()| {
            Ok(obj.vec.x)
        });

        methods.add_method("getY", |_, obj, ()| {
            Ok(obj.vec.y)
        });

        methods.add_method("getZ", |_, obj, ()| {
            Ok(obj.vec.z)
        });

        methods.add_method_mut("setX", |_, obj, x: f32| {
            Arc::make_mut(&mut obj.vec).x = x;
            Ok(())
        });

        methods.add_method_mut("setY", |_, obj, y: f32| {
            Arc::make_mut(&mut obj.vec).y = y;
            Ok(())
        });

        methods.add_method_mut("setZ", |_, obj, z: f32| {
            Arc::make_mut(&mut obj.vec).z = z;
            Ok(())
        });
    }
}

pub fn vec3_constructor(x: f32, y: f32, z: f32) -> LuaVec3 {
    LuaVec3 {
        vec: Arc::new(Vec3::new(x,y,z))
    }
}

#[derive(Clone)]
pub struct LuaQuat {
    pub quat: Arc<Quat>,
}

impl UserData for LuaQuat {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            Ok(format!("Transform {{ quat: {} }}", obj.quat))
        });

        methods.add_meta_function(MetaMethod::Eq, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(*a.quat == *b.quat)
        });

        methods.add_meta_function(MetaMethod::Lt, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(*a.quat < *b.quat)
        });

        methods.add_meta_function(MetaMethod::Le, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(*a.quat <= *b.quat)
        });

        methods.add_meta_function(MetaMethod::Add, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(LuaQuat { quat: Arc::new(*a.quat + *b.quat) })
        });

        methods.add_meta_function(MetaMethod::Sub, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(LuaQuat { quat: Arc::new(*a.quat - *b.quat) })
        });

        methods.add_meta_function(MetaMethod::Mul, |_, (a,b): (LuaQuat, LuaQuat)| {
            Ok(LuaQuat { quat: Arc::new(*a.quat * *b.quat) })
        });

        methods.add_meta_function(MetaMethod::Div, |_, (a,b): (LuaQuat, f32)| {
            Ok(LuaQuat { quat: Arc::new(*a.quat / b) })
        });

        methods.add_meta_function(MetaMethod::Unm, |_, a: LuaQuat| {
            Ok(LuaQuat { quat: Arc::new(-*a.quat) })
        });
    }
}

pub fn quat_euler_constructor(yaw: f32, pitch: f32, roll: f32) -> LuaQuat {
    LuaQuat {
        quat: Arc::new(Quat::from_rotation_ypr(yaw, pitch, roll))
    }
}

#[derive(Clone)]
pub struct LuaTransform {
    pub transform: Arc<Transform>,
}

impl LuaTransform {
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            transform: Arc::new(transform)
        }
    }
}

impl UserData for LuaTransform {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::ToString, |_, obj: Self| {
            let pos = obj.transform.pos;
            let rot = obj.transform.rot;
            let scale = obj.transform.scale;
            Ok(format!("Transform {{ pos: {} - rot: {} - scale: {} }}", pos, rot, scale))
        });

        methods.add_method("getRotation", |_, obj, ()| {
            Ok(LuaQuat { quat: Arc::new(obj.transform.rot) })
        });

        methods.add_method_mut("setRotation", |_, obj, rotation: LuaQuat| {
            Arc::make_mut(&mut obj.transform).rot = *rotation.quat;
            Ok(())
        });

        methods.add_method("getPosition", |_, obj, ()| {
            Ok(LuaVec3 { vec: Arc::new(obj.transform.pos) })
        });

        methods.add_method_mut("setPosition", |_, obj, rotation: LuaVec3| {
            Arc::make_mut(&mut obj.transform).pos = *rotation.vec;
            Ok(())
        });
    }
}

pub fn transform_constructor(pos: LuaVec3, rot: LuaQuat, scale: LuaVec3) -> LuaTransform {
    let transform = Transform::new(*pos.vec, *rot.quat, *scale.vec);
    LuaTransform {
        transform: Arc::new(transform)
    }
}
