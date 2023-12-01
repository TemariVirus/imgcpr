use crate::{nearest_color, Image, Pixel};
use image::Rgba;
use std::{
    io::{BufWriter, Write},
    path::Path,
};

pub fn save<P>(img: &Image, path: P) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::create(path)?;
    let mut stream = BufWriter::new(file);

    // Header
    let width = img.width().to_le_bytes();
    let height = img.height().to_le_bytes();
    stream.write_all(&width)?;
    stream.write_all(&height)?;

    const LOG2_PALETTE_SIZE: u8 = 3;
    stream.write_all(&[LOG2_PALETTE_SIZE, 0, 0, 0])?;

    let palette_size = 2u32.pow(LOG2_PALETTE_SIZE as u32);
    let palette = get_palette(img, palette_size);
    assert!(palette.len() == palette_size as usize);
    for &color in &palette {
        stream.write_all(&color.0)?;
    }

    // Data
    for &pixel in img.pixels() {
        let index = nearest_color(&palette, pixel);
        stream.write_all(&[index])?;
    }

    stream.flush()?;
    Ok(())
}

// TODO: create palette based on image
fn get_palette(img: &Image, palette_size: u32) -> Vec<Pixel> {
    vec![
        Rgba([0, 0, 0, 0]),
        Rgba([0, 0, 0, 255]),
        Rgba([0, 0, 255, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([0, 255, 255, 255]),
        Rgba([255, 0, 0, 255]),
        Rgba([255, 0, 255, 255]),
        Rgba([255, 255, 0, 255]),
    ]
}
