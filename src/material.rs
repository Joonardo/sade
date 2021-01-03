use crate::math::Vec3;
use crate::primitive::HitRecord;
use crate::ray::Ray;

use crate::texture::Texture;
use crate::EPSILON;
use rand::Rng;

#[derive(Clone)]
pub enum Material {
    Empty,
    Lambertian {
        albedo: Texture,
    },
    Metal {
        albedo: Texture,
        fuzz: f32,
    },
    Dielectric {
        fuzz: f32,
        albedo: Texture,
        ior: f32,
    },
    DiffuseLight {
        emit: Texture,
        intensity: f32,
    },
    Isotropic {
        albedo: Vec3,
    },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut dir = hit.normal() + Vec3::rand_in_unit_sphere(rng);

                if dir.near_zero() {
                    dir = hit.normal();
                }

                Some((
                    albedo(hit.uv(), hit.point()),
                    Ray {
                        origin: hit.point(),
                        dir,
                        t: ray_in.t,
                    },
                ))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(ray_in.dir.unit(), hit.normal());
                let mut dir = reflected + *fuzz * Vec3::rand_in_unit_sphere(rng);

                if dir.near_zero() {
                    dir = reflected;
                }

                if Vec3::dot(hit.normal(), dir) < 0. {
                    None
                } else {
                    Some((
                        albedo(hit.uv(), hit.point()),
                        Ray {
                            origin: hit.point(),
                            dir: dir.unit(),
                            t: ray_in.t,
                        },
                    ))
                }
            }
            Material::Dielectric { fuzz, albedo, ior } => {
                let ir = if hit.front_face() { 1. / *ior } else { *ior };

                let dir = ray_in.dir.unit();
                let n = hit.normal().unit();

                let cos_theta = Vec3::dot(-dir, n);

                let mut r = Vec3::refract(dir, n, ir)
                    .filter(|_| schlick_reflectance(cos_theta, ir) < rng.gen::<f32>())
                    .unwrap_or(Vec3::reflect(dir, n))
                    + *fuzz * Vec3::rand_in_unit_sphere(rng);

                if r.near_zero() {
                    r = hit.normal();
                }

                Some((
                    albedo(hit.uv(), hit.point()),
                    Ray {
                        origin: hit.point(),
                        dir: r.unit(),
                        t: ray_in.t,
                    },
                ))
            }
            Material::Isotropic { albedo } => Some((
                *albedo,
                Ray {
                    origin: hit.point(),
                    dir: Vec3::rand_in_unit_sphere(rng),
                    t: ray_in.t,
                },
            )),
            _ => None,
        }
    }

    pub fn emitted(&self, hit: &HitRecord) -> Vec3 {
        match self {
            Material::DiffuseLight { emit, intensity } => *intensity * emit(hit.uv(), hit.point()),
            _ => Vec3::new(0., 0., 0.),
        }
    }
}

fn schlick_reflectance(c: f32, ref_idx: f32) -> f32 {
    let r = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r * r;
    r0 + (1. - r0) * (1. - c).powi(5)
}
