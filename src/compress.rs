use crate::{
    color::{RGBf32, RGBu8},
    kmeans, Distance, Image, PaletteMethod,
};
use std::collections::HashMap;

pub fn compress(img: &Image, palette_method: PaletteMethod) -> Vec<u8> {
    const LOG2_PALETTE_SIZE: u8 = 4;

    let mut bytes = Vec::with_capacity(12 + 64 + (img.width() * img.height()) as usize);
    let pixels: Vec<_> = img.pixels().map(|&p| RGBf32::from(p)).collect();

    // Header
    let width = img.width().to_le_bytes();
    let height = img.height().to_le_bytes();
    bytes.extend_from_slice(&width);
    bytes.extend_from_slice(&height);

    let palette_size = 2u16.pow(LOG2_PALETTE_SIZE as u32);
    let palette = match palette_method {
        PaletteMethod::CIELab => unimplemented!(),
        PaletteMethod::Freq => get_palette_freq(&pixels, palette_size),
        PaletteMethod::KMeans => get_palette_k_means(&pixels, palette_size),
    };
    assert!(palette.len() <= palette_size as usize);
    bytes.extend_from_slice(&(palette.len() as u32).to_le_bytes());

    for &color in &palette {
        bytes.extend_from_slice(&color.0);
    }

    // Data
    for (i, pixel) in pixels.iter().enumerate() {
        let index = pixel.nearest_color(&palette);
        // Each index is 4 bits, so we can fit 2 indices in a byte
        if i % 2 == 0 {
            bytes.push(index);
        } else {
            *bytes.last_mut().unwrap() |= index << 4;
        }
    }

    bytes
}

/// Get a palette of the most frequently used colors in the image
fn get_palette_freq(pixels: &[RGBf32], palette_size: u16) -> Vec<RGBu8> {
    let palette_size = palette_size as usize;
    let mut palette: Vec<RGBu8> = Vec::with_capacity(palette_size);

    // Group and count colors
    let mut color_counts = HashMap::new();
    for &pixel in pixels.iter() {
        let count = color_counts.entry(pixel).or_insert(0u32);
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
            .map(|&p| p.into())
            // TODO: parameterise the treshold
            .any(|p: RGBf32| p.distance2(&color) < 0.015625)
        {
            continue;
        }

        palette.push(color.into());
    }

    palette
}

/// Get a palette by running k-means clustering on the image's colors
fn get_palette_k_means(pixels: &[RGBf32], palette_size: u16) -> Vec<RGBu8> {
    kmeans::fit(pixels, palette_size as usize, 10)
        .into_iter()
        .map(|p| p.into())
        .collect()
}
