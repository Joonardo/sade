use image::Rgba;
use std::ops::Index;

mod add;
mod div;
mod mul;
mod neg;
mod sub;

mod mat4;
mod vec3;
mod vec4;

#[derive(Debug, Copy, Clone)]
pub struct Mat4(pub [f32; 16]);

#[derive(Debug, Copy, Clone)]
pub struct Vec4([f32; 4]);

#[derive(Debug, Copy, Clone)]
pub struct Vec3([f32; 3]);

pub trait ZipMap {
    fn zip_map(&self, other: &Self, f: impl Fn(f32, f32) -> f32) -> Self;
    fn map(&self, f: impl Fn(f32) -> f32) -> Self;
}

pub trait Fold {
    fn fold(&self, init: f32, f: impl Fn(f32, f32) -> f32) -> f32;
}

#[derive(Copy, Clone)]
pub enum Axis3 {
    X,
    Y,
    Z,
}

impl Index<Axis3> for Vec3 {
    type Output = f32;

    fn index(&self, index: Axis3) -> &Self::Output {
        match index {
            Axis3::X => &self.0[0],
            Axis3::Y => &self.0[1],
            Axis3::Z => &self.0[2],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Axis4 {
    X,
    Y,
    Z,
    W,
}

impl Index<Axis4> for Vec4 {
    type Output = f32;

    fn index(&self, index: Axis4) -> &Self::Output {
        match index {
            Axis4::X => &self.0[0],
            Axis4::Y => &self.0[1],
            Axis4::Z => &self.0[2],
            Axis4::W => &self.0[3],
        }
    }
}

pub enum Channel {
    R,
    G,
    B,
}

impl Index<Channel> for Vec3 {
    type Output = f32;

    fn index(&self, idx: Channel) -> &Self::Output {
        match idx {
            Channel::R => &self.0[0],
            Channel::G => &self.0[1],
            Channel::B => &self.0[2],
        }
    }
}

impl From<Rgba<u8>> for Vec3 {
    fn from(v: Rgba<u8>) -> Self {
        Vec3::new(v.0[0] as f32, v.0[1] as f32, v.0[2] as f32) / 255.
    }
}
