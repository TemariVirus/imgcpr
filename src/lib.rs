pub mod color;
pub mod compress;
pub mod decompress;
mod kmeans;

use std::fmt::Debug;

use clap::ValueEnum;
use image::{ImageBuffer, Rgb};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

trait Distance
where
    Self: Sized,
{
    type Output: PartialOrd;

    /// Returns the distance
    fn distance(&self, other: &Self) -> Self::Output;

    /// Returns the squared distance
    fn distance2(&self, other: &Self) -> Self::Output;

    /// Returns the index of the nearest point
    fn nearest(&self, points: &[Self]) -> Option<usize> {
        if points.is_empty() {
            return None;
        }

        let mut nearest = 0;
        let mut nearest_distance = self.distance2(&points[0]);
        for (i, point) in points.iter().enumerate().skip(1) {
            let distance = self.distance2(point);
            if distance < nearest_distance {
                nearest = i;
                nearest_distance = distance;
            }
        }
        Some(nearest)
    }
}

trait Zero {
    fn zero() -> Self;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PaletteMethod {
    Freq,
    KMeans,
}
