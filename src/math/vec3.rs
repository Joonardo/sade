use crate::math::{Fold, Vec3, ZipMap};
use rand::Rng;
use std::ops::{Add, Range};

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        self.len() < 1e-10
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

    #[inline]
    pub fn dot(a: Self, b: Self) -> f32 {
        a.zip_map(&b, std::ops::Mul::mul)
            .fold(0., std::ops::Add::add)
    }

    #[inline]
    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y() * b.z() - a.z() * b.y(),
            a.z() * b.x() - a.x() * b.z(),
            a.x() * b.y() - a.y() * b.x(),
        )
    }

    #[inline]
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2. * Vec3::dot(v, n) * n
    }

    #[inline]
    pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = v.unit();
        let cos_theta = Vec3::dot(uv, n);

        let d = 1. - ni_over_nt * ni_over_nt * (1. - cos_theta * cos_theta);

        if d > 0. {
            Some(ni_over_nt * (uv - cos_theta * n) - d.sqrt() * n)
        } else {
            None
        }
    }

    #[inline]
    pub fn rand_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
        loop {
            let v = Vec3::new(rng.gen_range(-1. ..=1.), rng.gen_range(-1. ..=1.), 0.);
            if v.len() < 1. {
                return v;
            }
        }
    }

    #[inline]
    pub fn rand_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
        loop {
            let v = Vec3::new(
                rng.gen_range(-1. ..=1.),
                rng.gen_range(-1. ..=1.),
                rng.gen_range(-1. ..=1.),
            );
            if v.len() < 1. {
                return v;
            }
        }
    }

    #[inline]
    pub fn rand_in_unit_hemisphere(n: Vec3, rng: &mut impl Rng) -> Vec3 {
        let v = Vec3::rand_in_unit_sphere(rng);

        if Vec3::dot(v, n) < 0. {
            -v
        } else {
            v
        }
    }

    #[inline]
    pub fn rand(range: Range<f32>, rng: &mut impl Rng) -> Vec3 {
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
        )
    }
}

impl ZipMap for Vec3 {
    #[inline]
    fn zip_map(&self, other: &Self, f: impl Fn(f32, f32) -> f32) -> Self {
        Vec3::new(
            f(self.x(), other.x()),
            f(self.y(), other.y()),
            f(self.z(), other.z()),
        )
    }

    #[inline]
    fn map(&self, f: impl Fn(f32) -> f32) -> Self {
        Vec3::new(f(self.x()), f(self.y()), f(self.z()))
    }
}

impl Fold for Vec3 {
    #[inline]
    fn fold(&self, init: f32, f: impl Fn(f32, f32) -> f32) -> f32 {
        f(f(f(init, self.x()), self.y()), self.z())
    }
}

impl From<f32> for Vec3 {
    fn from(v: f32) -> Self {
        Vec3::new(v, v, v)
    }
}

impl std::iter::Sum for Vec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: std::iter::Iterator<Item = Self>,
    {
        iter.fold(Vec3::from(0.), Add::add)
    }
}
