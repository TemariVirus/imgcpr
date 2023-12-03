use crate::Image;
use image::Rgb;
use std::ops::{BitOrAssign, Shl};

pub fn decompress(bytes: &[u8]) -> Image {
    let mut bytes = bytes.iter().copied().peekable();
    let width: u32 = read(&mut bytes);
    let height: u32 = read(&mut bytes);

    let palette_size = read::<u32>(&mut bytes).try_into().unwrap();
    let mut palette: Vec<Rgb<u8>> = Vec::with_capacity(palette_size);
    for _ in 0..palette_size {
        let color = Rgb([read(&mut bytes), read(&mut bytes), read(&mut bytes)]);
        palette.push(color);
    }

    let mut img = Image::new(width, height);
    for (i, pixel) in img.pixels_mut().enumerate() {
        // TODO: Handle palatte sizes other than 16
        let index = if i % 2 == 0 {
            *bytes.peek().unwrap() & 0b00001111
        } else {
            bytes.next().unwrap() >> 4
        };
        *pixel = palette[usize::from(index)];
    }

    img
}

fn read<T>(bytes: &mut dyn Iterator<Item = u8>) -> T
where
    T: Default + Copy + From<u8> + Shl<usize, Output = T> + BitOrAssign<T>,
{
    let mut value = T::default();
    for i in 0..std::mem::size_of::<T>() {
        let byte: T = bytes.next().unwrap_or(0).into();
        value |= byte << (i * 8);
    }
    value
}
