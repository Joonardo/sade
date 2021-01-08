use crate::camera::Camera;
use crate::math::Vec3;
use crate::world::{ray_color, World};
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub struct Preview;

impl Preview {
    pub fn run(
        nx: usize,
        ny: usize,
        world: Arc<dyn World>,
        camera: Camera,
        background: Box<dyn Fn(Vec3) -> Vec3 + Sync + Send>,
    ) {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let surface = SurfaceTexture::new(nx as u32, ny as u32, &window);

        let pixels = Pixels::new(nx as u32, ny as u32, surface).unwrap();
        let pixels = Arc::new(Mutex::new(pixels));

        let calculate_pixels = pixels.clone();

        std::thread::spawn(move || {
            for s in 0.. {
                let layer = (0..ny)
                    .into_par_iter()
                    .rev()
                    .flat_map(|y| {
                        let mut rng = thread_rng();
                        (0..nx)
                            .map(|x| {
                                let u = (x as f32 + rng.gen::<f32>()) / (nx as f32 - 1.);
                                let v = (y as f32 + rng.gen::<f32>()) / (ny as f32 - 1.);

                                let ray = camera.get_ray(u, v, &mut rng);

                                ray_color(ray, &world, &background, &mut rng)
                            })
                            .collect::<Vec<Vec3>>()
                    })
                    .collect::<Vec<Vec3>>();

                {
                    let mut pixels = calculate_pixels.lock().unwrap();
                    let frame = pixels.get_frame();
                    for (c, pixel) in layer.iter().zip(frame.chunks_exact_mut(4)) {
                        let p = Vec3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32) / 255.;
                        let p = p * p;
                        pixel[3] = 0xff;

                        let p = 255. * ((p * (s as f32) + *c) / (s as f32 + 1.)).sqrt();

                        pixel[0] = p.x() as u8;
                        pixel[1] = p.y() as u8;
                        pixel[2] = p.z() as u8;
                    }
                }

                print!("\rsamples per pixel: {}", s + 1);
                window.request_redraw();
            }
            println!("\ndone.");
        });

        let mut input = WinitInputHelper::new();

        event_loop.run(move |event, _, control| {
            if let Event::RedrawRequested(_) = event {
                let mut pixels = pixels.lock().unwrap();
                if pixels.render().is_err() {
                    *control = ControlFlow::Exit;
                    return;
                }
            }

            if input.update(&event) {
                if input.key_held(VirtualKeyCode::A) {
                    println!("input");
                }
            }
        });
    }
}
