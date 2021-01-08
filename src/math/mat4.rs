use crate::math::{Mat4, ZipMap};

impl Mat4 {
    #[inline]
    pub fn eye() -> Self {
        Mat4([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ])
    }

    #[inline]
    pub fn sqrt(&self) -> Self {
        self.map(|v| v.sqrt())
    }

    #[inline]
    pub fn max(&self, other: Self) -> Self {
        self.zip_map(&other, f32::max)
    }

    #[inline]
    pub fn min(&self, other: Self) -> Self {
        self.zip_map(&other, f32::min)
    }

    #[inline]
    pub fn tr(&self) -> f32 {
        let mut acc = 0.;
        for i in 0..4 {
            acc += self.0[4 * i + i];
        }
        acc
    }

    #[inline]
    pub fn det(&self) -> f32 {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = self.0;

        a * (f * k * p + g * l * n + h * j * o - f * l * o - g * j * p - h * k * n)
            - b * (e * k * p + g * l * m + h * i * o - e * l * o - g * i * p - h * k * m)
            + c * (e * j * p + f * l * m + h * i * n - e * l * n - f * i * p - h * j * m)
            - d * (e * j * o + f * k * m + g * i * n - e * k * n - f * i * o - g * j * m)
    }

    #[inline]
    pub fn inv(&self) -> Option<Self> {
        let det = self.det();

        if det < 1e-10 {
            return None;
        }

        let a = *self;
        let a2 = a * a;
        let a3 = a * a2;

        let tr_a = a.tr();
        let tr_a2 = a2.tr();
        let tr_a3 = a3.tr();

        Some(
            1. / det
                * (1. / 6. * (tr_a * tr_a * tr_a - 3. * tr_a * tr_a2 + 2. * tr_a3) * Mat4::eye()
                    - 1. / 2. * (tr_a * tr_a - tr_a2) * a
                    + a2 * tr_a
                    - a3),
        )
    }
}

impl ZipMap for Mat4 {
    #[inline]
    fn zip_map(&self, other: &Self, f: impl Fn(f32, f32) -> f32) -> Self {
        let mut data = [0.; 16];
        for i in 0..16 {
            data[i] = f(self.0[i], other.0[i]);
        }
        Mat4(data)
    }

    #[inline]
    fn map(&self, f: impl Fn(f32) -> f32) -> Self {
        let mut data = [0.; 16];
        for i in 0..16 {
            data[i] = f(self.0[i]);
        }
        Mat4(data)
    }
}
