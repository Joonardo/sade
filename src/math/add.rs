use crate::math::{Mat4, Vec3, Vec4, ZipMap};
use std::ops::Add;

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Add::add)
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Add::add)
    }
}

impl Add for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Add::add)
    }
}
