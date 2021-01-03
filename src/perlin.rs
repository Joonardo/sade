use crate::math::{Fold, Vec3, ZipMap};
use rand::Rng;

pub struct Perlin<const N: usize> {
    points: [Vec3; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}

fn permute<T, const N: usize>(arr: &mut [T; N], rng: &mut impl Rng) {
    for i in (1..N).rev() {
        arr.swap(i, rng.gen_range(0..i));
    }
}

fn generate_perm<const N: usize>(rng: &mut impl Rng) -> [usize; N] {
    let mut perm = [0; N];

    for i in 0..N {
        perm[i] = i;
    }

    permute(&mut perm, rng);

    perm
}

#[allow(dead_code)]
fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], uvw: Vec3) -> f32 {
    let mut acc = 0.;

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                acc += (di as f32 * uvw.x() + (1. - di as f32) * (1. - uvw.x()))
                    * (dj as f32 * uvw.y() + (1. - dj as f32) * (1. - uvw.y()))
                    * (dk as f32 * uvw.z() + (1. - dk as f32) * (1. - uvw.z()))
                    * c[di][dj][dk];
            }
        }
    }
    acc
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f32 {
    let mut acc = 0.;

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let ijk = Vec3::new(di as f32, dj as f32, dk as f32);
                let weight = uvw - ijk;

                acc += (ijk * uvw + (Vec3::from(1.) - ijk) * (Vec3::from(1.) - uvw))
                    .fold(1., std::ops::Mul::mul)
                    * Vec3::dot(c[di][dj][dk], weight);
            }
        }
    }
    acc
}

impl<const N: usize> Perlin<N> {
    pub fn new(rng: &mut impl Rng) -> Self {
        let points = {
            let mut points = [Vec3::new(0., 0., 0.); N];

            for i in 0..N {
                points[i] = Vec3::rand(-1. ..1., rng);
            }

            points
        };

        let perm_x = generate_perm(rng);
        let perm_y = generate_perm(rng);
        let perm_z = generate_perm(rng);

        Perlin {
            points,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let ijk = p.map(f32::floor);
        let uvw = {
            let uvw = p - ijk;
            uvw * uvw * (Vec3::from(3.) - 2. * uvw)
        };

        let mut c = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.points[self.perm_x
                        [((ijk.x() as i32 + di as i32) & (N as i32 - 1)) as usize]
                        ^ self.perm_y[((ijk.y() as i32 + dj as i32) & (N as i32 - 1)) as usize]
                        ^ self.perm_z[((ijk.z() as i32 + dk as i32) & (N as i32 - 1)) as usize]];
                }
            }
        }

        perlin_interp(&c, uvw)

        /*
        let ijk = p.map(|c| (4. * c).floor());

        self.points[self.perm_x[(ijk.0 as i32 & (N as i32 - 1)) as usize]
            ^ self.perm_y[(ijk.1 as i32 & (N as i32 - 1)) as usize]
            ^ self.perm_z[(ijk.2 as i32 & (N as i32 - 1)) as usize]]
         */
    }

    pub fn turb(&self, mut p: Vec3, depth: usize) -> f32 {
        let mut acc = 0.;
        let mut weight = 1.;

        for _ in 0..depth {
            acc += weight * self.noise(p);
            weight *= 0.5;
            p = p * 2.;
        }
        acc.abs()
    }
}
