use crate::Image;
use image::Rgba;
use std::{
    ops::{BitOrAssign, Shl},
    slice::Iter,
};

pub fn decompress(bytes: &[u8]) -> Image {
    let mut bytes = bytes.iter();
    let width: u32 = read(&mut bytes);
    let height: u32 = read(&mut bytes);

    let log2_palette_size = read::<u32>(&mut bytes) as u8;
    let palette_size = 2usize.pow(log2_palette_size as u32);
    let mut palette = Vec::with_capacity(palette_size);
    for _ in 0..palette_size {
        let color: Rgba<u8> = Rgba([
            read(&mut bytes),
            read(&mut bytes),
            read(&mut bytes),
            read(&mut bytes),
        ]);
        palette.push(color);
    }

    let mut img = Image::new(width, height);
    for pixel in img.pixels_mut() {
        let index = read::<u8>(&mut bytes) as usize;
        *pixel = palette[index];
    }

    img
}

fn read<T>(bytes: &mut Iter<u8>) -> T
where
    T: Default + Copy + From<u8> + Shl<usize, Output = T> + BitOrAssign<T>,
{
    let mut value = T::default();
    for i in 0..std::mem::size_of::<T>() {
        let byte: T = (*bytes.next().unwrap_or(&0)).into();
        value |= byte << (i * 8);
    }
    value
}
