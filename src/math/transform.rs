use super::mat3::Mat3;
use super::vec2::Vec2;

pub struct Transform {
    pub pos: Vec2,
    pub rot: f32,
    pub scale: Vec2,

    matrix: Mat3,
    dirty: bool,
}

impl Transform {
    pub(crate) fn new() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            rot: 0.0,
            scale: Vec2::new(1.0, 1.0),
            matrix: Mat3::identity(),
            dirty: true,
        }
    }

    pub(crate) fn get_matrix(&mut self) -> Mat3 {
        if self.dirty {
            let t = Mat3::translation(self.pos.x, self.pos.y);
            let r = Mat3::rotation(self.rot);
            let s = Mat3::scale(self.scale.x, self.scale.y);
            let m = t.multiply(&r).multiply(&s);
            self.matrix = m;
            self.dirty = false;
        }
        self.matrix
    }

    pub(crate) fn set_position(&mut self, pos: Vec2) {
        if !self.pos.equal(pos) {
            self.pos = pos;
            self.dirty = true;
        }
    }

    pub(crate) fn set_rotation(&mut self, rot: f32) {
        if self.rot != rot {
            self.rot = rot;
            self.dirty = true;
        }
    }
    pub(crate) fn set_scale(&mut self, scale: Vec2) {
        if !self.scale.equal(scale) {
            self.scale = scale;
            self.dirty = true;
        }
    }
}
