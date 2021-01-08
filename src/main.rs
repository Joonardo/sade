use sade_h::bvh::Bvh;
use sade_h::camera::Camera;
use sade_h::image::Image;
use sade_h::material::Material;
use sade_h::math::{Axis3::*, Vec3};
use sade_h::primitive::{ConstantMedium, Hittable, LinearMove, Sphere, Transform, Triangle};
use sade_h::texture::{checker, image, marbled, perlin_turb, solid, Texture};

use rand::{thread_rng, Rng, SeedableRng};
use sade_h::mesh::Mesh;
use sade_h::preview::Preview;
use sade_h::world::Background;
use std::ops::Range;
use std::sync::Arc;

const SAMPLES: usize = 100;
const ASPECT_RATIO: f32 = 3. / 2.;
const WIDTH: usize = 1200;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;

type Scene = (Camera, Vec<Box<dyn Hittable>>, Background);

#[allow(dead_code)]
fn sphere_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(13., 2., 3.);
        let lookat = Vec3::new(0., 0., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            20.,
            ASPECT_RATIO,
            10.,
            0.1,
            exposure,
        )
    };

    let world = {
        let mat_ground = Material::Lambertian {
            albedo: checker(
                Texture::from(Vec3::new(0.2, 0.3, 0.1)),
                Texture::from(Vec3::new(0.9, 0.9, 0.9)),
            ),
        };
        let mat1 = Material::Lambertian {
            albedo: Texture::from(Vec3::new(0.4, 0.2, 0.1)),
        };
        let mat2 = Material::Dielectric {
            albedo: Texture::from(Vec3::new(1., 1., 1.)),
            fuzz: 0.0,
            ior: 1.5,
        };
        let mat3 = Material::Metal {
            albedo: Texture::from(Vec3::new(0.7, 0.6, 0.5)),
            fuzz: 0.,
        };

        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: mat_ground,
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(-4., 1., 0.),
            radius: 1.,
            material: mat1,
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(0., 1., 0.),
            radius: 1.0,
            material: mat2,
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(4., 1., 0.),
            radius: 1.0,
            material: mat3.clone(),
        }));

        let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
        for x in -11..=11 {
            for z in -11..=11 {
                let mat_selector = rng.gen::<f32>();
                let center = Vec3::new(
                    x as f32 + 0.1 + 0.8 * rng.gen::<f32>(),
                    0.2,
                    z as f32 + 0.1 + 0.8 * rng.gen::<f32>(),
                );

                if (center - Vec3::new(4., 0.2, 0.)).len() < 0.9 {
                    continue;
                }

                let material = {
                    if mat_selector < 0.8 {
                        Material::Lambertian {
                            albedo: Texture::from(
                                Vec3::rand(0.0..1.0, &mut rng) * Vec3::rand(0.0..1.0, &mut rng),
                            ),
                        }
                    } else if mat_selector < 0.95 {
                        Material::Metal {
                            albedo: Texture::from(Vec3::rand(0.5..1.0, &mut rng)),
                            fuzz: rng.gen_range(0. ..0.5),
                        }
                    } else {
                        Material::Dielectric {
                            ior: 1.5,
                            fuzz: 0.0,
                            albedo: Texture::from(Vec3::from(1.)),
                        }
                    }
                };

                let s = Sphere {
                    center,
                    radius: 0.2,
                    material,
                };

                if rng.gen::<f32>() > 0.5 {
                    world.push(Box::new(s));
                } else {
                    world.push(Box::new(LinearMove {
                        object: s,
                        velocity: Vec3::new(0., rng.gen_range(0.1..0.6), 0.),
                    }));
                };
            }
        }

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn perlin_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(13., 2., 3.);
        let lookat = Vec3::new(0., 0., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            20.,
            ASPECT_RATIO,
            10.,
            0.1,
            exposure,
        )
    };

    let world = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
        let mat = Material::Lambertian {
            albedo: marbled(4., &mut rng),
        };

        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: mat.clone(),
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: mat,
        }));

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn earth_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(13., 2., 3.);
        let lookat = Vec3::new(0., 2., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            30.,
            ASPECT_RATIO,
            10.,
            0.1,
            exposure,
        )
    };

    let world = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
        let mat_ground = Material::Lambertian {
            albedo: marbled(4., &mut rng),
        };
        let mat_earth = Material::Lambertian {
            albedo: image("./earthmap.jpg".to_string()),
        };

        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: mat_ground,
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: mat_earth,
        }));

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn earth_lights_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(13., 2., 3.);
        let lookat = Vec3::new(0., 2., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            30.,
            ASPECT_RATIO,
            10.,
            0.1,
            exposure,
        )
    };

    let world = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
        let mat_ground = Material::Lambertian {
            albedo: marbled(4., &mut rng),
        };
        let mat_earth = Material::Lambertian {
            albedo: image("./earthmap.jpg".to_string()),
        };
        let mat_light = Material::DiffuseLight {
            emit: solid(Vec3::new(1., 1., 1.)),
            intensity: 50.,
        };

        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: mat_ground,
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: mat_earth,
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(6., 6., 6.),
            radius: 0.5,
            material: mat_light,
        }));

        world
    };

    (camera, world, Box::new(|_| Vec3::new(0., 0., 0.)))
}

#[allow(dead_code)]
fn checker_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(13., 2., 3.);
        let lookat = Vec3::new(0., 0., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            20.,
            ASPECT_RATIO,
            10.,
            0.1,
            exposure,
        )
    };

    let world = {
        let albedo = checker(
            solid(Vec3::new(0.2, 0.3, 0.1)),
            solid(Vec3::new(0.9, 0.9, 0.9)),
        );

        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -10., 0.),
            radius: 10.,
            material: Material::Lambertian {
                albedo: albedo.clone(),
            },
        }));

        world.push(Box::new(Sphere {
            center: Vec3::new(0., 10., 0.),
            radius: 10.,
            material: Material::Lambertian { albedo },
        }));

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn triangle_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(0., 0., -3.);
        let lookat = Vec3::new(0., 0., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            45.,
            ASPECT_RATIO,
            10.,
            0.1,
            exposure,
        )
    };

    let world = {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -10.5, 0.),
            radius: 10.,
            material: Material::Lambertian {
                albedo: solid(Vec3::new(0.6, 0.4, 0.2)),
            },
        }));

        world.push(Box::new(Triangle {
            vertices: (Vec3::from(0.), Vec3::new(0., 1., 0.), Vec3::new(1., 0., 0.)),
            normals: (
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
            ),
            material: Material::Lambertian {
                albedo: solid(Vec3::new(1., 0., 0.)),
            },
        }));

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn bunny_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(0., -3., -20.);
        let lookat = Vec3::new(0., 0., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            30.,
            ASPECT_RATIO,
            (lookat - lookfrom).len(),
            0.01,
            exposure.clone(),
        )
    };

    let world = {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        let mut bunny = Mesh::load(
            "./assets/bunny-with-normals.obj".to_string(),
            &Transform::stack(
                [
                    Transform::rotate(0., -std::f32::consts::PI, 0.),
                    Transform::translate(Vec3::new(5., -5., -3.)),
                    Transform::scale(Vec3::from(8.)),
                ]
                .iter(),
            ),
            Material::Dielectric {
                albedo: solid(Vec3::new(1., 1., 1.)),
                fuzz: 0.0,
                ior: 1.5,
            },
        );

        world.append(&mut bunny);

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -1005., 0.),
            radius: 1000.,
            material: Material::Lambertian {
                albedo: perlin_turb(0.1, 7, &mut thread_rng()),
            },
        }));

        world.push(Box::new(Sphere {
            material: Material::Metal {
                albedo: image("./assets/earthmap.jpg".to_string()),
                fuzz: 0.7,
            },
            center: Vec3::new(6., 0., 0.),
            radius: 5.,
        }));

        world.push(Box::new(Sphere {
            material: Material::DiffuseLight {
                emit: solid(Vec3::from(1.)),
                intensity: 40.,
            },
            center: Vec3::new(3., 10., 5.),
            radius: 3.,
        }));

        world.push(Box::new(Sphere {
            material: Material::DiffuseLight {
                emit: solid(Vec3::from(1.)),
                intensity: 40.,
            },
            center: Vec3::new(-3., 3.5, -7.),
            radius: 1.,
        }));

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            0.2 * Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn constant_medium_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let lookfrom = Vec3::new(0., 3., -20.);
        let lookat = Vec3::new(0., 0., 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            30.,
            ASPECT_RATIO,
            (lookat - lookfrom).len(),
            0.01,
            exposure.clone(),
        )
    };

    let world = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
        let mat = Material::Lambertian {
            albedo: marbled(4., &mut rng),
        };

        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: Vec3::new(0., -1005., 0.),
            radius: 1000.,
            material: mat.clone(),
        }));

        world.push(Box::new(ConstantMedium::new(
            Box::new(Sphere {
                material: Material::Empty,
                center: Vec3::new(6., 0., 0.),
                radius: 5.,
            }),
            Material::Isotropic {
                albedo: Vec3::new(0.2, 0.4, 0.6),
            },
            0.5,
        )));

        let bunny = Mesh::load(
            "./bunny-with-normals.obj".to_string(),
            &Transform::stack(
                [
                    Transform::rotate(0., -std::f32::consts::PI, 0.),
                    Transform::translate(Vec3::new(5., -5., -3.)),
                    Transform::scale(Vec3::from(8.)),
                ]
                .iter(),
            ),
            Material::Empty,
        );

        world.push(Box::new(ConstantMedium::new(
            Box::new(Bvh::new(bunny, exposure.clone())),
            Material::Isotropic {
                albedo: Vec3::new(0.2, 0.4, 0.6),
            },
            0.5,
        )));

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

#[allow(dead_code)]
fn cornell_box_scene(exposure: Range<f32>) -> Scene {
    let camera = {
        let x = 2.;
        let y = 21.;
        let lookfrom = Vec3::new(x, y, -20.);
        let lookat = Vec3::new(x, y, 0.);

        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0., 1., 0.),
            90.,
            ASPECT_RATIO,
            (lookat - lookfrom).len(),
            0.01,
            exposure.clone(),
        )
    };

    let world = {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
        let mat = Material::Lambertian {
            albedo: marbled(2., &mut rng),
        };

        let world: Vec<Box<dyn Hittable>> = Mesh::load(
            "./assets/cornell-box.obj".to_string(),
            &Transform::stack(
                [
                    Transform::rotate(0., -std::f32::consts::PI, 0.),
                    Transform::scale(Vec3::from(8.)),
                ]
                .iter(),
            ),
            mat,
        );

        world
    };

    (
        camera,
        world,
        Box::new(|dir| {
            let t = 0.5 * (dir.unit()[Y] + 1.);
            Vec3::from(t) + (1. - t) * Vec3::new(0.5, 0.7, 1.)
        }),
    )
}

/* fn load_scene(path: String) -> Scene {
    ron::from_str(&*std::fs::read_to_string(path.as_str()).expect("Couldn't load scene."))
        .expect("Failed to deserialize scene.")
} */

fn main() {
    eprintln!(
        "{:?}",
        (0..10)
            .collect::<Vec<u32>>()
            .chunks(4)
            .collect::<Vec<&[u32]>>()
    );

    let exposure = 0f32..1.;

    // let (camera, world, background) = sphere_scene(exposure.clone());
    // let (camera, world) = checker_scene(exposure.clone());
    // let (camera, world, background) = perlin_scene(exposure.clone());
    // let (camera, world, background) = earth_scene(exposure.clone());
    // let (camera, world, background) = earth_lights_scene(exposure.clone());
    // let (camera, world, background) = triangle_scene(exposure.clone());
    let (camera, world, background) = bunny_scene(exposure.clone());
    // let (camera, world, background) = constant_medium_scene(exposure.clone());
    // let (camera, world, background) = cornell_box_scene(exposure.clone());

    // let mut rng = rand::rngs::StdRng::seed_from_u64(0xAA33EBC);
    // let image = Image::cast(WIDTH, HEIGHT, 10, &camera, &world[..], &mut rng);

    let world = Bvh::new(world, exposure.clone());

    // let image = Image::par_cast(WIDTH, HEIGHT, SAMPLES, &camera, background, world);
    // image.print_ppm();

    Preview::run(WIDTH, HEIGHT, Arc::new(world), camera, background);
}
