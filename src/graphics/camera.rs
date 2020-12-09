use glam::*;

use crate::math::Transform;

/// Camera struct, both for orthographic and perspective
pub struct Camera {
    pub ortho: bool,
    pub transform: Transform,
    pub fov: f32, //In radians
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(ortho: bool, transform: Transform, fov: f32) -> Self {
        if fov >= 180.0 || fov <= 0.0 {
            panic!("FOV cannot be 180 (or greater) or 0 (or less)!");
        }
        Self {
            ortho: ortho,
            transform: transform,
            fov: fov,

            near: 0.02,
            far: 1024.0,
        }
    }

    pub fn get_proj(&self) -> Mat4 {
        let (width, height) = {
            let surface = unsafe { &crate::ROCK.as_ref().unwrap().surface };
            let size = surface.window().size();
            (size.0 as f32, size.1 as f32)
        };
        let aspect_ratio = width / height;
        if !self.ortho {
            Mat4::perspective_rh_gl(
                self.fov,
                aspect_ratio,
                self.near,
                self.far,
            )
        } else {
            Mat4::orthographic_rh_gl(
                -aspect_ratio,
                aspect_ratio,
                -1.0,
                 1.0,
                -1.0, //Near
                 1.0, //Far
            )
        }
    }

    pub fn get_view(&self) -> Mat4 {
        Mat4::from_rotation_translation(
            self.transform.rot,
            self.transform.pos,
        )
    }
}
