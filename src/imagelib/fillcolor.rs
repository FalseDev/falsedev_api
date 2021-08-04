use image::{Rgb, RgbImage};

pub fn fill_color(color: [u8; 3], size: (u32, u32)) -> RgbImage {
    let mut img = RgbImage::new(size.0, size.1);

    for x in 0..size.0 {
        for y in 0..size.1 {
            img.put_pixel(x, y, Rgb(color));
        }
    }
    img
}
