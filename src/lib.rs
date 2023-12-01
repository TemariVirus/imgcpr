pub mod compress;
pub mod decompress;

use image::{ImageBuffer, Rgba};

type Pixel = Rgba<u8>;
type Image = ImageBuffer<Pixel, Vec<u8>>;

pub fn nearest_color(palette: &[Pixel], pixel: Pixel) -> u8 {
    2
}
