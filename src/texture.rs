use crate::math::Vec3;
use crate::perlin::Perlin;
use image::GenericImageView;
use rand::Rng;
use std::sync::Arc;

pub type Texture = Arc<dyn Fn(Vec3, Vec3) -> Vec3 + Send + Sync>;

pub fn solid(color: Vec3) -> Texture {
    Arc::new(move |_, _| color)
}

pub fn checker(even: Texture, odd: Texture) -> Texture {
    Arc::new(move |uv, p| {
        let t = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();

        if t < 0. {
            odd(uv, p)
        } else {
            even(uv, p)
        }
    })
}

pub fn perlin_noise(scale: f32, rng: &mut impl Rng) -> Texture {
    let perlin: Perlin<256> = Perlin::new(rng);

    Arc::new(move |_, p| Vec3::from(0.5 * (1.0 + perlin.noise(scale * p))))
}

pub fn perlin_turb(scale: f32, depth: usize, rng: &mut impl Rng) -> Texture {
    let perlin: Perlin<256> = Perlin::new(rng);

    Arc::new(move |_, p| Vec3::from(perlin.turb(scale * p, depth)))
}

pub fn marbled(scale: f32, rng: &mut impl Rng) -> Texture {
    let perlin: Perlin<256> = Perlin::new(rng);

    Arc::new(move |_, p| Vec3::from(0.5 * (1.0 + (scale * p.z() + 10. * perlin.turb(p, 7)).sin())))
}

pub fn image(path: String) -> Texture {
    let img = image::io::Reader::open(&path)
        .expect(format!("Image {} not found", path).as_str())
        .decode()
        .unwrap();

    Arc::new(move |uv, _| {
        Vec3::from(img.get_pixel(
            (uv.x() * (img.dimensions().0 - 1) as f32) as u32,
            ((1. - uv.y()) * (img.dimensions().1 - 1) as f32) as u32,
        ))
    })
}

impl From<Vec3> for Texture {
    fn from(c: Vec3) -> Self {
        solid(c)
    }
}
