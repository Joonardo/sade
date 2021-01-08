use crate::bvh::Bvh;
use crate::math::Vec3;
use crate::primitive::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::{EPSILON, MAX_BOUNCES};
use rand::Rng;
use std::sync::Arc;

pub trait World: Send + Sync {
    fn trace(&self, ray: &Ray, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord>;
}

pub type Background = Box<dyn Fn(Vec3) -> Vec3 + Sync + Send>;

pub fn ray_color(
    mut ray: Ray,
    world: &impl World,
    background: &Background,
    rng: &mut impl Rng,
) -> Vec3 {
    let mut bounces = 0;
    let mut acc = Vec3::from(0.);
    let mut strength = Vec3::from(1.);

    while let Some(hit) = world.trace(&ray, &mut || rng.gen()) {
        acc = acc + strength * hit.material().emitted(&hit);

        if let Some((attenuation, r)) = hit.material().scatter(&ray, &hit, rng) {
            strength = strength * attenuation;
            ray = r;
        } else {
            break;
        }

        bounces += 1;
        if bounces > MAX_BOUNCES {
            return acc;
        }
    }
    acc + strength * background(ray.dir.unit())
}

impl World for Arc<dyn World> {
    fn trace(&self, ray: &Ray, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        (**self).trace(ray, rng)
    }
}

impl World for &[Arc<dyn Hittable>] {
    fn trace(&self, ray: &Ray, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        let mut nearest_t = f32::INFINITY;
        let mut nearest = None;

        for obj in self.iter() {
            if let Some(h) = obj.hit(ray, EPSILON..nearest_t, rng) {
                if h.t() < nearest_t {
                    nearest_t = h.t();
                    nearest = Some(h);
                }
            }
        }

        nearest
    }
}

impl World for Bvh {
    fn trace(&self, ray: &Ray, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        self.hit(ray, EPSILON..f32::INFINITY, rng)
    }
}
