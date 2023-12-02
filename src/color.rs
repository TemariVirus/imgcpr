use crate::{Distance, Zero};
use core::iter::Sum;
use image::Rgb;
use std::{
    hash::{Hash, Hasher},
    ops::{AddAssign, Div, Index},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RGBu8(pub [u8; 3]);

impl Index<usize> for RGBu8 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<RGBf32> for RGBu8 {
    fn from(rgb: RGBf32) -> Self {
        RGBu8([
            (rgb[0] * 255.0).round() as u8,
            (rgb[1] * 255.0).round() as u8,
            (rgb[2] * 255.0).round() as u8,
        ])
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBf32(pub [f32; 3]);

impl RGBf32 {
    // TODO: use CIELab color space
    /// Returns the index of the nearest color in the palette
    pub fn nearest_color(&self, palette: &[RGBu8]) -> u8 {
        palette
            .iter()
            .enumerate()
            .min_by(|(_, &p1), (_, &p2)| {
                self.distance2(&p1.into())
                    .partial_cmp(&self.distance2(&p2.into()))
                    .unwrap()
            })
            .unwrap()
            .0 as u8
    }
}

impl AddAssign for RGBf32 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs[0];
        self.0[1] += rhs[1];
        self.0[2] += rhs[2];
    }
}

impl Div<f32> for RGBf32 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        RGBf32([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl Index<usize> for RGBf32 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Sum for RGBf32 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = [0f32; 3];
        for rgb in iter {
            sum[0] += rgb[0];
            sum[1] += rgb[1];
            sum[2] += rgb[2];
        }
        RGBf32(sum)
    }
}

impl Distance for RGBf32 {
    type Output = f32;

    fn distance(&self, other: &Self) -> Self::Output {
        self.distance2(other).sqrt()
    }

    fn distance2(&self, other: &Self) -> Self::Output {
        let p1 = [self[0], self[1], self[2]];
        let p2 = [other[0], other[1], other[2]];

        // Euclidean distance squared
        let dr = p1[0] - p2[0];
        let dg = p1[1] - p2[1];
        let db = p1[2] - p2[2];
        dr * dr + dg * dg + db * db
    }
}

impl Zero for RGBf32 {
    fn zero() -> Self {
        RGBf32([0.0, 0.0, 0.0])
    }
}

impl From<Rgb<u8>> for RGBf32 {
    fn from(rgb: Rgb<u8>) -> Self {
        RGBf32([
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        ])
    }
}

impl From<RGBu8> for RGBf32 {
    fn from(rgb: RGBu8) -> Self {
        RGBf32([
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        ])
    }
}

impl Eq for RGBf32 {}

impl Hash for RGBf32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[0].to_bits().hash(state);
        self.0[1].to_bits().hash(state);
        self.0[2].to_bits().hash(state);
    }
}
