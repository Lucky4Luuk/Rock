use glam::*;

#[derive(Copy, Clone)]
pub struct Transform {
    pos: Vec3,
    rot: Quat,
    scale: Vec3,
}

impl Transform {
    pub fn new(pos: Vec3, rot: Quat, scale: Vec3) -> Self {
        Self {
            pos: pos,
            rot: rot,
            scale: scale,
        }
    }

    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn rot(&self) -> Quat {
        self.rot
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rot, self.pos)
    }
}
