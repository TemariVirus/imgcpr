use std::collections::HashMap;

use crate::{nearest_color, Distance, Image, Pixel};
use image::Rgba;

pub fn compress(img: &Image) -> Vec<u8> {
    const LOG2_PALETTE_SIZE: u8 = 4;

    let mut bytes = Vec::with_capacity(12 + 64 + (img.width() * img.height()) as usize);

    // Header
    let width = img.width().to_le_bytes();
    let height = img.height().to_le_bytes();
    bytes.extend_from_slice(&width);
    bytes.extend_from_slice(&height);

    let palette_size = 2usize.pow(LOG2_PALETTE_SIZE as u32);
    let palette = get_palette(img, palette_size);
    assert!(palette.len() <= palette_size);
    bytes.extend_from_slice(&(palette.len() as u32).to_le_bytes());

    for &color in &palette {
        bytes.extend_from_slice(&color.0);
    }

    // Data
    for (i, &pixel) in img.pixels().enumerate() {
        let index = nearest_color(&palette, pixel);
        // Each index is 4 bits, so we can fit 2 indices in a byte
        if i % 2 == 0 {
            bytes.push(index);
        } else {
            *bytes.last_mut().unwrap() |= index << 4;
        }
    }

    bytes
}

fn get_palette(img: &Image, palette_size: usize) -> Vec<Pixel> {
    let mut palette = Vec::with_capacity(palette_size);
    palette.push(Rgba([0, 0, 0, 0]));

    // Group and count colors
    let mut color_counts = HashMap::new();
    for &pixel in img.pixels() {
        let color = [pixel[0] >> 4 << 4, pixel[1] >> 4 << 4, pixel[2] >> 4 << 4];
        let count = color_counts.entry(color).or_insert(0);
        *count += 1;
    }
    // Collect and sort in ascending order
    let mut colors: Vec<_> = color_counts.into_iter().collect();
    colors.sort_unstable_by(|(_, count1), (_, count2)| count1.cmp(count2));

    while palette.len() < palette_size {
        let (color, _) = match colors.pop() {
            Some(color) => color,
            None => break,
        };

        // Skip color if it's too close to another color in the palette
        if palette
            .iter()
            .map(|&p| [p[0], p[1], p[2]])
            .any(|p| p.distance(&color) < 32f64.powi(2))
        {
            continue;
        }

        palette.push(Rgba([color[0], color[1], color[2], 255]));
    }

    palette
}
