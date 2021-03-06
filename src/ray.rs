use crate::math::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub t: f32,
}

impl Ray {
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}
