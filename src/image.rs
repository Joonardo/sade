use crate::camera::Camera;
use crate::math::{Channel::*, Vec3};
use crate::world::{ray_color, Background, World};

use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::sync::Mutex;

pub struct Image(Vec<Vec<Vec3>>);

impl Image {
    pub fn cast(
        nx: usize,
        ny: usize,
        ns: usize,
        camera: Camera,
        background: Background,
        world: impl World,
        rng: &mut impl Rng,
    ) {
        Image::compute(nx, ny, |x, y| {
            (0..ns)
                .map(|_| {
                    let u = (x as f32 + rng.gen::<f32>()) / (nx as f32 - 1.);
                    let v = (y as f32 + rng.gen::<f32>()) / (ny as f32 - 1.);

                    let ray = camera.get_ray(u, v, rng);

                    ray_color(ray, &world, &background, rng)
                })
                .sum::<Vec3>()
                / (ns as f32)
        });
    }

    fn compute(nx: usize, ny: usize, mut f: impl FnMut(usize, usize) -> Vec3) -> Image {
        Image(
            (0..ny)
                .rev()
                .map(|y| {
                    eprintln!("\rscanlines: {} / {}", ny - y - 1, ny);
                    (0..nx).map(|x| f(x, y)).collect()
                })
                .collect(),
        )
    }

    pub fn par_cast(
        nx: usize,
        ny: usize,
        ns: usize,
        camera: &Camera,
        background: Background,
        world: impl World,
    ) -> Image {
        Image::par_compute(nx, ny, |x, y| {
            let mut rng = thread_rng();

            (0..ns)
                .map(|_| {
                    let u = (x as f32 + rng.gen::<f32>()) / (nx as f32 - 1.);
                    let v = (y as f32 + rng.gen::<f32>()) / (ny as f32 - 1.);

                    let ray = camera.get_ray(u, v, &mut rng);

                    ray_color(ray, &world, &background, &mut rng)
                })
                .sum::<Vec3>()
                / (ns as f32)
        })
    }

    fn par_compute(nx: usize, ny: usize, f: impl Fn(usize, usize) -> Vec3 + Sync) -> Image {
        let progress_counter = Mutex::new(0_usize);
        Image(
            (0..ny)
                .into_par_iter()
                .rev()
                .map(|y| {
                    {
                        let mut progress = match progress_counter.lock() {
                            Ok(data) => data,
                            Err(e) => e.into_inner(),
                        };
                        *progress += 1;
                        eprint!("\rscanlines: {} / {}", *progress, ny);
                    }
                    (0..nx).map(|x| f(x, y)).collect()
                })
                .collect(),
        )
    }

    pub fn print_ppm(self) {
        println!("P3\n{}\t{}\n255", self.0[0].len(), self.0.len());
        for row in self.0 {
            for c in row {
                let color = c.sqrt();

                fn to_u8(v: f32) -> i32 {
                    ((255.999 * v) as i32).max(0).min(255)
                }

                println!(
                    "{}\t{}\t{}",
                    to_u8(color[R]),
                    to_u8(color[G]),
                    to_u8(color[B])
                );
            }
        }
    }
}
