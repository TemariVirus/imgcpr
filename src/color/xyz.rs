use super::{CieLab, RgbU8};
use std::ops::Index;

/// XYZ color space with values scaled to [0, 1]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Xyz(pub [f32; 3]);

impl Index<usize> for Xyz {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<RgbU8> for Xyz {
    fn from(rgb: RgbU8) -> Self {
        // Adjust RGB values
        let rgb = rgb.0.map(|x| {
            let x = x as f32 / 255.0;
            if x <= 0.04045 {
                x / 12.92
            } else {
                ((x + 0.055) / 1.055).powf(2.4)
            }
        });

        Xyz([
            0.4124564 * rgb[0] + 0.3575761 * rgb[1] + 0.1804375 * rgb[2],
            0.2126729 * rgb[0] + 0.7151522 * rgb[1] + 0.0721750 * rgb[2],
            0.0193339 * rgb[0] + 0.119192_ * rgb[1] + 0.9503041 * rgb[2],
        ])
    }
}

impl From<CieLab> for Xyz {
    fn from(cielab: CieLab) -> Self {
        let y = (cielab[0] + 16.0) / 116.0;
        let xyz = [y + cielab[1] / 500.0, y, y - cielab[2] / 200.0];

        // Un-adjust XYZ values
        Xyz(xyz.map(|x| {
            if x > 0.206893 {
                x.powi(3)
            } else {
                0.1284185 * (x - 16.0 / 116.0)
            }
        }))
    }
}
