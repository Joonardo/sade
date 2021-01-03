use crate::math::{Mat4, Vec3, Vec4, ZipMap};
use std::ops::Sub;

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Sub::sub)
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Sub::sub)
    }
}

impl Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Sub::sub)
    }
}
