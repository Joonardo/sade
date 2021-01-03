use crate::math::{Fold, Vec3, Vec4, ZipMap};
use std::ops::Add;

impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4([x, y, z, w])
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn w(&self) -> f32 {
        self.0[3]
    }

    #[inline]
    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn sqrt(&self) -> Self {
        self.map(|v| v.sqrt())
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        self.zip_map(&other, f32::max)
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        self.zip_map(&other, f32::min)
    }

    #[inline]
    pub fn len_sqr(&self) -> f32 {
        self.map(|v| v * v).fold(0., Add::add)
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    #[inline]
    pub fn unit(&self) -> Self {
        *self / self.len()
    }
}

impl ZipMap for Vec4 {
    #[inline]
    fn zip_map(&self, other: &Self, f: impl Fn(f32, f32) -> f32) -> Self {
        Vec4::new(
            f(self.x(), other.x()),
            f(self.y(), other.y()),
            f(self.z(), other.z()),
            f(self.w(), other.w()),
        )
    }

    #[inline]
    fn map(&self, f: impl Fn(f32) -> f32) -> Self {
        Vec4::new(f(self.x()), f(self.y()), f(self.z()), f(self.w()))
    }
}

impl Fold for Vec4 {
    #[inline]
    fn fold(&self, init: f32, f: impl Fn(f32, f32) -> f32) -> f32 {
        f(f(f(f(init, self.x()), self.y()), self.z()), self.w())
    }
}

impl From<f32> for Vec4 {
    fn from(v: f32) -> Self {
        Vec4::new(v, v, v, v)
    }
}

impl std::iter::Sum for Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: std::iter::Iterator<Item = Self>,
    {
        iter.fold(Vec4::from(0.), Add::add)
    }
}
