use super::{
    lms::{Lms, NonLinearLms},
    RgbU8,
};
use crate::{Distance, Zero};
use image::Rgb;
use std::{
    hash::{Hash, Hasher},
    iter::Sum,
    ops::{AddAssign, Div, Index},
};

// https://www.itu.int/dms_pubrec/itu-r/rec/bt/R-REC-BT.2124-0-201901-I!!PDF-E.pdf
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Itp(pub [f32; 3]);

impl AddAssign for Itp {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs[0];
        self.0[1] += rhs[1];
        self.0[2] += rhs[2];
    }
}

impl Div<f32> for Itp {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Itp([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl Index<usize> for Itp {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Sum for Itp {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = [0f32; 3];
        for rgb in iter {
            sum[0] += rgb[0];
            sum[1] += rgb[1];
            sum[2] += rgb[2];
        }
        Itp(sum)
    }
}

impl Distance for Itp {
    type Output = f32;

    fn distance(&self, other: &Self) -> Self::Output {
        720.0 * self.distance2(other).sqrt()
    }

    fn distance2(&self, other: &Self) -> Self::Output {
        // Euclidean distance squared
        let dr = self[0] - other[0];
        let dg = self[1] - other[1];
        let db = self[2] - other[2];
        dr * dr + dg * dg + db * db
    }
}

impl Zero for Itp {
    fn zero() -> Self {
        Itp([0.0, 0.0, 0.0])
    }
}

impl From<Rgb<u8>> for Itp {
    fn from(rgb: Rgb<u8>) -> Self {
        let rgb: RgbU8 = rgb.into();
        let lms: Lms = rgb.into();
        let nllms: NonLinearLms = lms.into();
        nllms.into()
    }
}

impl From<NonLinearLms> for Itp {
    fn from(nllms: NonLinearLms) -> Self {
        Itp([
            0.5 * nllms[0] + 0.5 * nllms[1],
            // 0.806884765625, 1.6617431640625, 0.8548583984375
            0.80688477 * nllms[0] - 1.6617432 * nllms[1] + 0.8548584 * nllms[2],
            // 4.378173828125, 4.24560546875, 0.132568359375
            4.378174 * nllms[0] - 4.2456055 * nllms[1] - 0.13256836 * nllms[2],
        ])
    }
}

impl Eq for Itp {}

impl Hash for Itp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[0].to_bits().hash(state);
        self.0[1].to_bits().hash(state);
        self.0[2].to_bits().hash(state);
    }
}
