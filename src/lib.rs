pub mod color;
pub mod compress;
pub mod decompress;
mod kmeans;

use clap::ValueEnum;
use image::{ImageBuffer, Rgb};

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

trait Distance {
    type Output;

    fn distance(&self, other: &Self) -> Self::Output;

    fn distance2(&self, other: &Self) -> Self::Output;
}

trait Zero {
    fn zero() -> Self;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PaletteMethod {
    CIELab,
    Freq,
    KMeans,
}
