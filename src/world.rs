use crate::bvh::Bvh;
use crate::primitive::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::EPSILON;

pub trait World: Send + Sync {
    fn trace(&self, ray: &Ray, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord>;
}

impl World for &[Box<dyn Hittable>] {
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
