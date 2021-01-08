use crate::math::{Mat4, Vec3, Vec4};
use crate::ray::Ray;

use crate::aabb::AABB;
use crate::material::Material;
use crate::EPSILON;
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord<'m> {
    t: f32,
    point: Vec3,
    normal: Vec3,
    material: &'m Material,
    uv: Vec3,
    front_face: bool,
}

impl<'m> HitRecord<'m> {
    pub fn new(ray: &Ray, t: f32, n: Vec3, uv: Vec3, material: &'m Material) -> Self {
        let point = ray.at(t);
        let front_face = Vec3::dot(ray.dir, n) < 0.;

        HitRecord {
            t,
            point,
            normal: if front_face { n.unit() } else { -n.unit() },
            front_face,
            material,
            uv,
        }
    }

    #[inline]
    pub fn t(&self) -> f32 {
        self.t
    }

    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    #[inline]
    pub fn point(&self) -> Vec3 {
        self.point
    }

    #[inline]
    pub fn material(&'m self) -> &'m Material {
        self.material
    }

    #[inline]
    pub fn front_face(&'m self) -> bool {
        self.front_face
    }

    #[inline]
    pub fn uv(&self) -> Vec3 {
        self.uv
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t: Range<f32>, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord>;
    fn bounding_box(&self, exposure: Range<f32>) -> AABB;
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t: Range<f32>, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        (**self).hit(ray, t, rng)
    }

    #[inline]
    fn bounding_box(&self, exposure: Range<f32>) -> AABB {
        (**self).bounding_box(exposure)
    }
}

impl Hittable for Arc<dyn Hittable> {
    fn hit(&self, ray: &Ray, t: Range<f32>, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        (**self).hit(ray, t, rng)
    }

    #[inline]
    fn bounding_box(&self, exposure: Range<f32>) -> AABB {
        (**self).bounding_box(exposure)
    }
}

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
    pub material: Material,
}

impl Hittable for Sphere {
    #[inline]
    fn hit(&self, ray: &Ray, t: Range<f32>, _: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_sqr();
        let hb = Vec3::dot(oc, ray.dir);
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = hb * hb - a * c;

        if discriminant < 0.0 {
            None
        } else {
            for tc in &[
                (-hb - discriminant.sqrt()) / a,
                (-hb + discriminant.sqrt()) / a,
            ] {
                if *tc <= t.end && *tc >= t.start {
                    let n = (ray.at(*tc) - self.center) / self.radius;
                    let n = n.unit();
                    let u = ((-n.z()).atan2(n.x()) + std::f32::consts::PI)
                        / (2. * std::f32::consts::PI);
                    let v = (-n.y()).acos() / std::f32::consts::PI;

                    return Some(HitRecord::new(
                        ray,
                        *tc,
                        n,
                        Vec3::new(u, v, 0.),
                        &self.material,
                    ));
                }
            }
            None
        }
    }

    #[inline]
    fn bounding_box(&self, _: Range<f32>) -> AABB {
        AABB {
            min: self.center - Vec3::from(self.radius),
            max: self.center + Vec3::from(self.radius),
        }
    }
}

pub struct LinearMove<H> {
    pub object: H,
    pub velocity: Vec3,
}

impl<H: Hittable> Hittable for LinearMove<H> {
    #[inline]
    fn hit(&self, ray: &Ray, t: Range<f32>, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        self.object.hit(
            &Ray {
                origin: ray.origin - self.velocity * ray.t,
                ..*ray
            },
            t,
            rng,
        )
    }

    #[inline]
    fn bounding_box(&self, exposure: Range<f32>) -> AABB {
        let bb = self.object.bounding_box(exposure.clone());

        let bb0 = AABB {
            min: bb.min + self.velocity * exposure.start,
            max: bb.max + self.velocity * exposure.start,
        };
        let bb1 = AABB {
            min: bb.min + self.velocity * exposure.end,
            max: bb.max + self.velocity * exposure.end,
        };

        AABB::merge(&bb0, &bb1)
    }
}

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Material,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t: Range<f32>, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        let mut h1 = self.boundary.hit(ray, -f32::INFINITY..f32::INFINITY, rng)?;
        let mut h2 = self.boundary.hit(ray, h1.t + EPSILON..f32::INFINITY, rng)?;

        h1.t = h1.t.max(t.start);
        h2.t = h2.t.min(t.end);

        if h1.t >= h2.t {
            return None;
        }

        let distance_inside_boundary = (h2.t - h1.t) * ray.dir.len();
        let hit_distance = self.neg_inv_density * rng().log2();

        if distance_inside_boundary < hit_distance {
            return None;
        }

        Some(HitRecord::new(
            ray,
            h1.t + hit_distance / ray.dir.len(),
            h1.normal,
            h1.uv,
            &self.phase_function,
        ))
    }

    fn bounding_box(&self, exposure: Range<f32>) -> AABB {
        self.boundary.bounding_box(exposure)
    }
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, phase_function: Material, density: f32) -> Self {
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density: -1. / density,
        }
    }
}

pub struct Triangle {
    pub vertices: (Vec3, Vec3, Vec3),
    pub normals: (Vec3, Vec3, Vec3),
    pub material: Material,
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_range: Range<f32>, _: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        let (v1, v2, v3) = self.vertices;

        let edge1 = v2 - v1;
        let edge2 = v3 - v1;

        let h = Vec3::cross(ray.dir, edge2);
        let a = Vec3::dot(edge1, h);

        if a > -EPSILON / 2. && a < EPSILON / 2. {
            return None;
        }

        let f = 1. / a;
        let s = ray.origin - v1;
        let u = f * Vec3::dot(s, h);

        if u < 0. || u > 1. {
            return None;
        }

        let q = Vec3::cross(s, edge1);
        let v = f * Vec3::dot(ray.dir, q);

        if v < 0.0 || u + v > 1. {
            return None;
        }

        let t = f * Vec3::dot(edge2, q);

        if t_range.start <= t && t <= t_range.end {
            let (n1, n2, n3) = self.normals;

            Some(HitRecord::new(
                ray,
                t,
                n1 * (1. - u - v) + u * n2 + v * n3,
                Vec3::new(u, v, 0.),
                &self.material,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, _: Range<f32>) -> AABB {
        // Add small offset to ensure that the box has non-zero dimensions
        let offset = Vec3::from(EPSILON / 2.);
        let min = self.vertices.0.min(self.vertices.1).min(self.vertices.2) - offset;
        let max = self.vertices.0.max(self.vertices.1).max(self.vertices.2) + offset;

        AABB { min, max }
    }
}

pub struct Transform(pub Mat4);

impl Transform {
    pub fn apply(&self, p: Vec4) -> Vec4 {
        self.0 * p
    }

    pub fn identity() -> Self {
        Transform(Mat4::eye())
    }

    pub fn stack<'m>(ts: impl Iterator<Item = &'m Transform>) -> Transform {
        Transform(ts.fold(Mat4::eye(), |acc, t| acc * t.0))
    }

    pub fn rotate(a: f32, b: f32, c: f32) -> Transform {
        let (sa, ca) = a.sin_cos();
        let (sb, cb) = b.sin_cos();
        let (sc, cc) = c.sin_cos();

        Transform(Mat4([
            ca * cb,
            ca * sb * sc - sa * cc,
            ca * sb * cc + sa * sc,
            0.,
            sa * cb,
            sa * sb * sc + ca * cc,
            sa * sb * cc - ca * sc,
            0.,
            -sb,
            cb * sc,
            cb * cc,
            0.,
            0.,
            0.,
            0.,
            1.,
        ]))
    }

    pub fn translate(t: Vec3) -> Transform {
        Transform(Mat4([
            1.,
            0.,
            0.,
            t.x(),
            0.,
            1.,
            0.,
            t.y(),
            0.,
            0.,
            1.,
            t.z(),
            0.,
            0.,
            0.,
            1.,
        ]))
    }

    pub fn scale(s: Vec3) -> Transform {
        Transform(Mat4([
            s.x(),
            0.,
            0.,
            0.,
            0.,
            s.y(),
            0.,
            0.,
            0.,
            0.,
            s.z(),
            0.,
            0.,
            0.,
            0.,
            1.,
        ]))
    }
}
