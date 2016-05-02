extern crate image;

mod color;
mod hitable;
mod ray;
mod sphere;
mod vector;

use std::f64;
use std::fs::File;
use std::path::Path;
use image::{
    GenericImage,
    ImageBuffer
};

use color::Color;
use hitable::Hitable;
use ray::Ray;
use sphere::Sphere;
use vector::Vector;

fn unit_vector(vector: &Vector) -> Vector {
    return vector / vector.length();
}

fn color(ray: Ray, hitable: &Hitable) -> Color {
    let (hit, n) = hitable.hit(&ray, 0.0, f64::MAX);
    if hit {
        let c = 0.5 * &Vector{x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0};
        return Color{r: c.x, g: c.y, b: c.z};
    }
    let unit_direction = unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    let v = &((1.0 - t) * &Vector{x: 1.0, y: 1.0, z: 1.0})
        + &(t * &Vector{x: 0.5, y: 0.7, z: 1.0});
    Color{r: v.x, g: v.y, b: v.z}
}

fn main() {
    let nx = 200;
    let ny = 100;

    // consider:
    // let buffer: &[u8] = ...;
    // image::save_buffer(&Path::new("image.png"), buffer, 800, 600, image::RGB(8))

    let mut img = ImageBuffer::new(nx, ny);

    let lower_left_corner = Vector {x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vector {x: 4.0, y: 0.0, z: 0.0};
    let vertical = Vector {x: 0.0, y: 2.0, z: 0.0};
    let origin = Vector {x: 0.0, y: 0.0, z: 0.0};

    let sphere1 = Sphere{center: &Vector{x: 0.0, y: 0.0, z: -1.0}, radius: 0.5};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray{a: &origin, b: &(&(&lower_left_corner + &(u * &horizontal)) + &(v * &vertical))};
            let col = color(r, &sphere1);

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
