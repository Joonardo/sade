use crate::math::{Mat4, Vec3, Vec4, ZipMap};
use std::ops::Neg;

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl Neg for Mat4 {
    type Output = Mat4;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
