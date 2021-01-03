use crate::aabb::AABB;
use crate::bvh::Bvh;
use crate::material::Material;
use crate::math::{Vec3, Vec4};
use crate::primitive::{HitRecord, Triangle};
use crate::primitive::{Hittable, Transform};
use crate::ray::Ray;
use std::ops::Range;

pub struct Mesh(pub Bvh);

impl Mesh {
    pub fn load(
        path: String,
        transform: &Transform,
        material: Material,
        exposure: Range<f32>,
    ) -> Self {
        let (models, _materials) = tobj::load_obj(&path, true)
            .expect(format!("No obj-file \"{}\" found.", &path).as_str());

        let mut objs: Vec<Box<dyn Hittable>> = vec![];

        for m in models.iter() {
            if m.mesh.normals.is_empty() {
                eprintln!(
                    "No normals found for mesh {} at {}. Your mileage may vary.",
                    &m.name, &path
                );
            }

            let mut next_face = 0;
            for f in 0..m.mesh.num_face_indices.len() {
                let end = next_face + m.mesh.num_face_indices[f] as usize;
                let face_indices: Vec<_> = m.mesh.indices[next_face..end].iter().collect();

                // We only handle triangles
                assert_eq!(face_indices.len(), 3);

                let (i, j, k) = (
                    *face_indices[0] as usize,
                    *face_indices[1] as usize,
                    *face_indices[2] as usize,
                );
                let v = &m.mesh.positions;

                let v1 = {
                    let v = transform.apply(Vec4::new(v[3 * i], v[3 * i + 1], v[3 * i + 2], 1.));
                    v.xyz() / v.w()
                };
                let v2 = {
                    let v = transform.apply(Vec4::new(v[3 * j], v[3 * j + 1], v[3 * j + 2], 1.));
                    v.xyz() / v.w()
                };
                let v3 = {
                    let v = transform.apply(Vec4::new(v[3 * k], v[3 * k + 1], v[3 * k + 2], 1.));
                    v.xyz() / v.w()
                };

                let normals = if !m.mesh.normals.is_empty() {
                    let n = &m.mesh.normals;
                    (
                        transform
                            .apply(Vec4::new(n[3 * i], n[3 * i + 1], n[3 * i + 2], 0.))
                            .xyz()
                            .unit(),
                        transform
                            .apply(Vec4::new(n[3 * j], n[3 * j + 1], n[3 * j + 2], 0.))
                            .xyz()
                            .unit(),
                        transform
                            .apply(Vec4::new(n[3 * k], n[3 * k + 1], n[3 * k + 2], 0.))
                            .xyz()
                            .unit(),
                    )
                } else {
                    let n = Vec3::cross(v3 - v1, v2 - v1);
                    (n, n, n)
                };

                objs.push(Box::new(Triangle {
                    vertices: (v1, v2, v3),
                    normals,
                    material: material.clone(),
                }));

                next_face = end;
            }
        }

        eprintln!("loaded {} tris: {}", path, objs.len());

        Mesh(Bvh::new(objs, exposure))
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t: Range<f32>, rng: &mut dyn FnMut() -> f32) -> Option<HitRecord> {
        self.0.hit(ray, t, rng)
    }

    fn bounding_box(&self, exposure: Range<f32>) -> AABB {
        self.0.bounding_box(exposure)
    }
}
