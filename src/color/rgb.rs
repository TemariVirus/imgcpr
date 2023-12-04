use super::{
    cielab::CieLab,
    lms::{Lms, NonLinearLms},
    xyz::Xyz,
    Itp,
};
use crate::Distance;
use image::Rgb;
use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RgbU8(pub [u8; 3]);

impl Index<usize> for RgbU8 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Distance for RgbU8 {
    type Output = f32;

    fn distance(&self, other: &Self) -> Self::Output {
        self.distance2(other).sqrt()
    }

    fn distance2(&self, other: &Self) -> Self::Output {
        // Redmean
        // https://www.compuphase.com/cmetric.htm#:~:text=A%20low%2Dcost%20approximation
        let r_mean = (i32::from(self[0]) + i32::from(other[0])) / 2;
        let dr = i32::from(self[0]) - i32::from(other[0]);
        let dg = i32::from(self[1]) - i32::from(other[1]);
        let db = i32::from(self[2]) - i32::from(other[2]);

        let dr2 = ((512 + r_mean) * dr * dr) as f32 / 256.0;
        let dg2 = (4 * dg * dg) as f32;
        let db2 = ((767 - r_mean) * db * db) as f32 / 256.0;
        (dr2 + dg2 + db2) / 3.0 // Normalise to [0, 255*3]
    }
}

impl From<Rgb<u8>> for RgbU8 {
    fn from(rgb: Rgb<u8>) -> Self {
        RgbU8(rgb.0)
    }
}

impl From<Lms> for RgbU8 {
    fn from(lms: Lms) -> Self {
        let lms = lms.0.map(|lms| lms * 255.0);
        RgbU8([
            (3.43661 * lms[0] - 2.50645 * lms[1] + 0.0698454 * lms[2]).round() as u8,
            (-0.79133 * lms[0] + 1.9836 * lms[1] - 0.192271 * lms[2]).round() as u8,
            (-0.0259499 * lms[0] - 0.0989137 * lms[1] + 1.12486 * lms[2]).round() as u8,
        ])
    }
}

impl From<Xyz> for RgbU8 {
    fn from(xyz: Xyz) -> Self {
        let rgb = [
            3.24045 * xyz[0] - 1.53714 * xyz[1] - 0.498532 * xyz[2],
            -0.969266 * xyz[0] + 1.87601 * xyz[1] + 0.0415561 * xyz[2],
            0.0556434 * xyz[0] - 0.204026 * xyz[1] + 1.05723 * xyz[2],
        ];

        // Un-adjust RGB values
        RgbU8(rgb.map(|x| {
            let x = if x <= 0.0031308 {
                x * 12.92
            } else {
                1.055 * x.powf(1.0 / 2.4) - 0.055
            };
            (x * 255.0).round() as u8
        }))
    }
}

impl From<Itp> for RgbU8 {
    fn from(itp: Itp) -> Self {
        let nllms = NonLinearLms::from(itp);
        let lms = Lms::from(nllms);
        lms.into()
    }
}

impl From<CieLab> for RgbU8 {
    fn from(cielab: CieLab) -> Self {
        let xyz = Xyz::from(cielab);
        xyz.into()
    }
}
