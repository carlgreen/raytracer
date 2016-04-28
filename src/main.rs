extern crate image;

use std::fs::File;
use std::path::Path;
use image::{
    GenericImage,
    ImageBuffer
};

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn main() {
    let nx = 200;
    let ny = 100;

    // consider:
    // let buffer: &[u8] = ...;
    // image::save_buffer(&Path::new("image.png"), buffer, 800, 600, image::RGB(8))

    let mut img = ImageBuffer::new(nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Color {
                r: i as f64 / nx as f64,
                g: j as f64 / ny as f64,
                b: 0.2,
            };
            let ir = (255.99 * col.r) as u8;
            let ig = (255.99 * col.g) as u8;
            let ib = (255.99 * col.b) as u8;
            let pixel = image::Rgb([ir, ig, ib]);
            img.put_pixel(i, j, pixel);
        }
    }

    let ref mut fout = File::create(&Path::new("target/image.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}
