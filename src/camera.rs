use crate::math::{Axis3::*, Vec3};
use crate::ray::Ray;
use rand::Rng;

use std::ops::Range;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,

    exposure: Range<f32>,

    #[allow(unused)]
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        focal_length: f32,
        aperture: f32,
        exposure: Range<f32>,
    ) -> Self {
        let fov_rad = vfov / 180.0 * std::f32::consts::PI;
        let h = (fov_rad / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focal_length * viewport_width * u;
        let vertical = focal_length * viewport_height * v;
        let lower_left = origin - (vertical + horizontal) / 2.0 - focal_length * w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left,
            lens_radius: aperture / 2.0,
            exposure,
            w,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk(rng);
        let offset = self.u * rd[X] + self.v * rd[Y];
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
            t: rng.gen_range(self.exposure.clone()),
        }
    }
}
