use glam::*;

#[derive(Copy, Clone)]
pub struct Transform {
    pub pos: Vec3,
    pub rot: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(pos: Vec3, rot: Quat, scale: Vec3) -> Self {
        Self {
            pos: pos,
            rot: rot,
            scale: scale,
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rot, self.pos)
    }

    pub fn get_normal_matrix(&self) -> Mat4 {
        self.get_matrix().inverse().transpose()
    }
}
