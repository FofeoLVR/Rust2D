use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub(crate) fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    pub(crate) fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub(crate) fn length(&self) -> f32 {
        self.dot(&self).sqrt()
    }
    pub(crate) fn normalized(&self) -> Option<Vec2> {
        let length = self.length();
        if length == 0.0 {
            return None;
        }
        Some(Vec2 {
            x: self.x / length,
            y: self.y / length,
        })
    }

    pub(crate) fn distance(&self, other: &Vec2) -> f32 {
        (*self - *other).length()
    }

    pub(crate) fn lerp(&self, b: Vec2, factor: f32) -> Vec2 {
        *self + (b - *self) * factor
    }

    pub(crate) fn angle(&self) -> f32 {
        self.y.atan2(self.x).to_degrees()
    }

    pub(crate) fn rotate(&self, angle: f32) -> Vec2 {
        let a = angle.to_radians();
        let cos = a.cos();
        let sin = a.sin();

        Vec2::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    pub(crate) fn reflect(&self, normal: &Vec2) -> Vec2 {
        let y = match normal.normalized() {
            Some(n) => n,
            None => return Vec2::new(0.0, 0.0),
        };

        let dot = self.dot(&y);
        let x = 2.0 * (dot);

        *self - y * x
    }

    pub(crate) fn equal(&self, compared: Vec2) -> bool {
        self.x == compared.x && self.y == compared.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Self::Output {
        Vec2::new((self.x + other.x), (self.y + other.y))
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Self::Output {
        Vec2::new((self.x - other.x), (self.y - other.y))
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2::new((self.x * other.x), (self.y * other.y))
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}
