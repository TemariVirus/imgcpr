pub mod compress;
pub mod decompress;

use image::{ImageBuffer, Rgba};

type Pixel = Rgba<u8>;
type Image = ImageBuffer<Pixel, Vec<u8>>;

trait Distance<T> {
    fn distance(&self, other: &T) -> f64;
}

impl Distance<Pixel> for Pixel {
    // TODO: use a better estimation of color distance
    fn distance(&self, other: &Pixel) -> f64 {
        // Multiply RGB by alpha
        let p1 = [
            self[0] as f64,
            self[1] as f64,
            self[2] as f64,
            self[3] as f64,
        ];
        let p2 = [
            other[0] as f64,
            other[1] as f64,
            other[2] as f64,
            other[3] as f64,
        ];

        // Euclidean distance squared
        let dr = p1[0] * p1[3] - p2[0] * p2[3];
        let dg = p1[1] * p1[3] - p2[1] * p2[3];
        let db = p1[2] * p1[3] - p2[2] * p2[3];
        // Alpha is in range [0, 255], so value is scaled by 255^2
        dr * dr + dg * dg + db * db
    }
}

impl Distance<[u8; 3]> for [u8; 3] {
    fn distance(&self, other: &[u8; 3]) -> f64 {
        let p1 = [self[0] as f64, self[1] as f64, self[2] as f64];
        let p2 = [other[0] as f64, other[1] as f64, other[2] as f64];

        // Euclidean distance squared
        let dr = p1[0] - p2[0];
        let dg = p1[1] - p2[1];
        let db = p1[2] - p2[2];
        dr * dr + dg * dg + db * db
    }
}

pub fn nearest_color(palette: &[Pixel], pixel: Pixel) -> u8 {
    palette
        .iter()
        .enumerate()
        .min_by(|(_, p1), (_, p2)| pixel.distance(p1).partial_cmp(&pixel.distance(p2)).unwrap())
        .unwrap()
        .0 as u8
}
