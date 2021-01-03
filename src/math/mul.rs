use crate::math::{Mat4, Vec3, Vec4, ZipMap};
use std::ops::Mul;

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Mul::mul)
    }
}

impl Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_map(&rhs, Mul::mul)
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    data[4 * i + j] += self.0[4 * i + k] * rhs.0[4 * k + j];
                }
            }
        }
        Mat4(data)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|v| rhs * v)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|v| rhs * v)
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|v| rhs * v)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.map(|v| self * v)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs.map(|v| self * v)
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs.map(|v| self * v)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut data = [0.; 4];
        for i in 0..4 {
            for j in 0..4 {
                data[i] += self.0[4 * i + j] * rhs.0[j];
            }
        }
        Vec4(data)
    }
}
