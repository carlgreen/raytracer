extern crate image;

use std::fs::File;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
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

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<'a> Add<&'a Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;

    fn div(self, div: f64) -> Vector {
        Vector { x: self.x / div, y: self.y / div, z: self.z / div }
    }
}

impl<'a> Mul<&'a Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: &Vector) -> Vector {
        Vector { x: self * vector.x, y: self * vector.y, z: self * vector.z }
    }
}

impl<'a> Sub<&'a Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

struct Ray<'a> {
    a: &'a Vector,
    b: &'a Vector,
}

impl<'a> Ray<'a> {
    fn origin(&self) -> Vector {
        Vector{x: self.a.x, y: self.a.y, z: self.a.z}
    }

    fn direction(&self) -> Vector {
        Vector{x: self.b.x, y: self.b.y, z: self.b.z}
    }

    fn point_at_parameter(&self, t: f64) -> Vector {
        Vector { x: self.a.x + t * self.b.x, y: self.a.y + t * self.b.y, z: self.a.z + t * self.b.z }
    }
}

fn unit_vector(vector: &Vector) -> Vector {
    return vector / vector.length();
}

fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn hit_sphere(center: &Vector, radius: f64, ray: &Ray) -> f64 {
    let oc = &ray.origin() - center;
    let a = dot(&ray.direction(), &ray.direction());
    let b = 2.0 * dot(&oc, &ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn color(ray: Ray) -> Color {
    let t = hit_sphere(&Vector{x: 0.0, y: 0.0, z: -1.0}, 0.5, &ray);
    if t > 0.0 {
        let n = unit_vector(&(&ray.point_at_parameter(t) - &Vector{x: 0.0, y: 0.0, z: -1.0}));
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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray{a: &origin, b: &(&(&lower_left_corner + &(u * &horizontal)) + &(v * &vertical))};
            let col = color(r);

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
