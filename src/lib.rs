#![feature(min_const_generics)]

pub const EPSILON: f32 = 0.001;
pub const MAX_BOUNCES: usize = 5;

mod aabb;
pub mod bvh;
pub mod camera;
pub mod image;
pub mod material;
pub mod math;
pub mod mesh;
mod perlin;
pub mod preview;
pub mod primitive;
pub mod ray;
pub mod texture;
pub mod world;
