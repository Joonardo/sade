use crate::math::{Mat4, Vec3, Vec4};
use std::ops::Div;

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl Div<f32> for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}
