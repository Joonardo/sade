use crate::math::{Axis3::*, Vec3};
use crate::ray::Ray;
use std::ops::Range;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, mut t_range: Range<f32>) -> Option<Range<f32>> {
        for dim in &[X, Y, Z] {
            let inv_d = 1.0 / ray.dir[*dim];
            let mut t0 = (self.min[*dim] - ray.origin[*dim]) * inv_d;
            let mut t1 = (self.max[*dim] - ray.origin[*dim]) * inv_d;

            if inv_d < 0.0 {
                let c = t0;
                t0 = t1;
                t1 = c;
            }

            t_range.start = t0.max(t_range.start);
            t_range.end = t1.min(t_range.end);

            if t_range.end < t_range.start {
                return None;
            }
        }
        Some(t_range)
    }

    pub fn merge(a: &AABB, b: &AABB) -> AABB {
        AABB {
            max: a.max.max(b.max),
            min: a.min.min(b.min),
        }
    }
}
