use super::{xyz::Xyz, RgbU8};
use crate::{Distance, Zero};
use image::Rgb;
use std::{
    hash::{Hash, Hasher},
    iter::Sum,
    ops::{AddAssign, Div, Index},
};

// https://en.wikipedia.org/wiki/CIELAB_color_space
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CieLab(pub [f32; 3]);

impl AddAssign for CieLab {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs[0];
        self.0[1] += rhs[1];
        self.0[2] += rhs[2];
    }
}

impl Div<f32> for CieLab {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        CieLab(self.0.map(|x| x / rhs))
    }
}

impl Index<usize> for CieLab {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Sum for CieLab {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = [0f32; 3];
        for rgb in iter {
            sum[0] += rgb[0];
            sum[1] += rgb[1];
            sum[2] += rgb[2];
        }
        CieLab(sum)
    }
}

impl Distance for CieLab {
    type Output = f32;

    fn distance(&self, other: &Self) -> Self::Output {
        self.distance2(other).sqrt()
    }

    fn distance2(&self, other: &Self) -> Self::Output {
        // TODO: use CIEDE2000 for better results
        let dl = self[0] - other[0];
        let da = self[1] - other[1];
        let db = self[2] - other[2];
        dl * dl + da * da + db * db
    }
}

impl Zero for CieLab {
    fn zero() -> Self {
        CieLab([0.0, 0.0, 0.0])
    }
}

impl From<Rgb<u8>> for CieLab {
    fn from(rgb: Rgb<u8>) -> Self {
        let rgb = RgbU8::from(rgb);
        let xyz = Xyz::from(rgb);
        xyz.into()
    }
}

impl From<Xyz> for CieLab {
    fn from(xyz: Xyz) -> Self {
        // Adjust XYZ values
        let xyz = xyz.0.map(|x| {
            if x > 0.008856 {
                x.powf(1.0 / 3.0)
            } else {
                7.78704 * x + 16.0 / 116.0
            }
        });

        let l = 116.0 * xyz[1] - 16.0;
        let a = 500.0 * (xyz[0] - xyz[1]);
        let b = 200.0 * (xyz[1] - xyz[2]);

        CieLab([l, a, b])
    }
}

impl Eq for CieLab {}

impl Hash for CieLab {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[0].to_bits().hash(state);
        self.0[1].to_bits().hash(state);
        self.0[2].to_bits().hash(state);
    }
}
