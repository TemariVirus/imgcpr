use crate::{
    color::{Itp, RgbU8},
    kmeans, Distance, Image, PaletteMethod,
};
use std::collections::HashMap;

const LOG2_PALETTE_SIZE: u8 = 4;

pub fn compress(img: &Image, palette_method: PaletteMethod) -> Vec<u8> {
    let width = usize::try_from(img.width()).unwrap();
    let height = usize::try_from(img.height()).unwrap();
    let mut bytes = Vec::with_capacity(12 + 48 + (width * height).div_ceil(2));

    // Header
    let width = img.width().to_le_bytes();
    let height = img.height().to_le_bytes();
    bytes.extend_from_slice(&width);
    bytes.extend_from_slice(&height);

    match palette_method {
        PaletteMethod::Freq => compress_freq(&mut bytes, img),
        PaletteMethod::KMeans => compress_k_means(&mut bytes, img),
    }

    bytes
}

fn compress_freq(bytes: &mut Vec<u8>, img: &Image) {
    let palette_size = 2u16.pow(LOG2_PALETTE_SIZE.into());
    let palette = get_palette_freq(img, palette_size);

    // Header
    bytes.extend_from_slice(&u32::try_from(palette.len()).unwrap().to_le_bytes());
    for &color in &palette {
        bytes.extend_from_slice(&color.0);
    }

    // Data
    for (i, &pixel) in img.pixels().enumerate() {
        let pixel: RgbU8 = pixel.into();
        // TODO: use dithering
        let index = pixel.nearest(&palette).unwrap();
        let index = u8::try_from(index).unwrap();
        // Each index is 4 bits, so we can fit 2 indices in a byte
        if i % 2 == 0 {
            bytes.push(index);
        } else {
            *bytes.last_mut().unwrap() |= index << 4;
        }
    }
}

fn compress_k_means(bytes: &mut Vec<u8>, img: &Image) {
    let palette_size = 2u16.pow(LOG2_PALETTE_SIZE.into());
    let pixels: Vec<Itp> = img.pixels().map(|&p| p.into()).collect();
    let palette = get_palette_k_means(&pixels, palette_size);

    // Header
    bytes.extend_from_slice(&u32::try_from(palette.len()).unwrap().to_le_bytes());
    for &color in &palette {
        let color: RgbU8 = color.into();
        bytes.extend_from_slice(&color.0);
    }

    // Data
    for (i, pixel) in pixels.iter().enumerate() {
        // TODO: use dithering
        let index = pixel.nearest(&palette).unwrap();
        let index = u8::try_from(index).unwrap();
        // Each index is 4 bits, so we can fit 2 indices in a byte
        if i % 2 == 0 {
            bytes.push(index);
        } else {
            *bytes.last_mut().unwrap() |= index << 4;
        }
    }
}

/// Get a palette of the most frequently used colors in the image
fn get_palette_freq(img: &Image, palette_size: u16) -> Vec<RgbU8> {
    let palette_size = palette_size.into();
    let mut palette: Vec<RgbU8> = Vec::with_capacity(palette_size);

    // Group and count colors
    let mut color_counts = HashMap::new();
    for pixel in img.pixels() {
        let key = RgbU8([pixel[0] & 0xf0, pixel[1] & 0xf0, pixel[2] & 0xf0]);
        let count = color_counts.entry(key).or_insert(0u32);
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
            .any(|p| { p.distance2(&color) } < 32f32.powi(2))
        {
            continue;
        }

        palette.push(color);
    }

    palette
}

/// Get a palette by running k-means clustering on the image's colors
fn get_palette_k_means(pixels: &[Itp], palette_size: u16) -> Vec<Itp> {
    // TODO: Compare with CIEDE2000
    kmeans::fit(pixels, palette_size.into(), 0.00005, 250)
}
