use crate::aabb::AABB;
use crate::math::{Axis3::*, Vec3};
use crate::primitive::{HitRecord, Hittable};
use crate::ray::Ray;
use rand::Rng;
use std::ops::Range;

pub struct Bvh {
    bounding_box: AABB,
    children: BvhChildren,
}

enum BvhChildren {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

impl Hittable for Bvh {
    fn hit(
        &self,
        ray: &Ray,
        mut t_range: Range<f32>,
        rng: &mut dyn FnMut() -> f32,
    ) -> Option<HitRecord> {
        match self.bounding_box.hit(ray, t_range.clone()) {
            None => None,
            Some(t) => match &self.children {
                BvhChildren::Leaf(obj) => {
                    obj.hit(ray, t.start.max(t_range.start)..t.end.min(t_range.end), rng)
                }
                BvhChildren::Node { left, right } => {
                    let left_hit = left.hit(ray, t_range.clone(), rng);

                    if let Some(h) = &left_hit {
                        t_range.end = h.t();
                    }

                    match (left_hit, right.hit(ray, t_range, rng)) {
                        (None, None) => None,
                        (None, o) | (o, None) => o,
                        (Some(h1), Some(h2)) => {
                            if h1.t() < h2.t() {
                                Some(h1)
                            } else {
                                Some(h2)
                            }
                        }
                    }
                }
            },
        }
    }

    fn bounding_box(&self, _: Range<f32>) -> AABB {
        self.bounding_box
    }
}

impl Bvh {
    pub fn new(mut objs: Vec<Box<dyn Hittable>>, exposure: Range<f32>) -> Self {
        let (children, bounding_box) = match objs.len() {
            0 => panic!("Cannot construct BVH with zero objects."),
            1 => {
                let obj = objs.pop().unwrap();
                let bb = obj.bounding_box(exposure.clone());
                (BvhChildren::Leaf(obj), bb)
            }
            _ => {
                let (max, min) = objs.iter().fold(
                    (Vec3::from(-f32::INFINITY), Vec3::from(f32::INFINITY)),
                    |(max, min), obj| {
                        let bb = obj.bounding_box(exposure.clone());

                        (max.max(bb.max), min.min(bb.min))
                    },
                );

                let max_dim = {
                    let range = max - min;

                    let mut max_dim = X;
                    let mut max_range = range[X];

                    for ax in &[Y, Z] {
                        if range[*ax] > max_range {
                            max_dim = *ax;
                            max_range = range[*ax];
                        }
                    }

                    max_dim
                };

                objs.sort_unstable_by(|a, b| {
                    let abb = a.bounding_box(exposure.clone());
                    let bbb = b.bounding_box(exposure.clone());

                    let ac = abb.min[max_dim] + abb.max[max_dim];
                    let bc = bbb.min[max_dim] + bbb.max[max_dim];

                    ac.partial_cmp(&bc).unwrap()
                });

                let pivot = objs.len() / 2;

                /*
                let avg = objs.iter().map(|o| {
                    let bb = o.bounding_box(exposure.clone());
                    bb.max[max_dim] + bb.min[max_dim]
                }).sum::<f32>() / objs.len() as f32;

                let pivot = objs.partition_point(|o| {
                    let bb = o.bounding_box(exposure.clone());
                    bb.max[max_dim] + bb.min[max_dim] < avg
                });
                */

                let left = Bvh::new(objs.drain(pivot..).collect(), exposure.clone());
                let right = Bvh::new(objs, exposure.clone());

                let bb = AABB::merge(
                    &left.bounding_box(exposure.clone()),
                    &right.bounding_box(exposure.clone()),
                );

                (
                    BvhChildren::Node {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bb,
                )
            }
        };

        Bvh {
            children,
            bounding_box,
        }
    }
}
