extern crate image;
extern crate rand;

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vector;

use std::f64;
use std::fs::File;
use std::path::Path;
use image::ImageBuffer;

use camera::Camera;
use color::Color;
use hitable::Hitable;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use ray::Ray;
use sphere::Sphere;
use vector::Vector;

fn color(ray: Ray, hitable: &Hitable, depth: u16) -> Color {
    let (hit, _, _, _, yes, attenuation, scattered) = hitable.hit(&ray, 0.0, f64::MAX);
    if hit {
        if depth < 50 {
            if yes {
                return &attenuation * &color(scattered, hitable, depth + 1);
            }
        }
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }
    let unit_direction = &ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let v = &((1.0 - t) * &Vector::new_ones_vector()) + &(t * &Vector {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    });
    Color {
        r: v.x,
        g: v.y,
        b: v.z,
    }
}

fn make_scene() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();
    let sphere1 = Sphere {
        center: Box::new(Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vector {
                x: 0.1,
                y: 0.2,
                z: 0.5,
            },
        }),
    };
    world.push(Box::new(sphere1));
    let sphere2 = Sphere {
        center: Box::new(Vector {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        }),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vector {
                x: 0.8,
                y: 0.8,
                z: 0.0,
            },
        }),
    };
    world.push(Box::new(sphere2));
    let sphere3 = Sphere {
        center: Box::new(Vector {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        }),
        radius: 0.5,
        material: Box::new(Metal::new(
            &Vector {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
            0.0,
        )),
    };
    world.push(Box::new(sphere3));
    let sphere4 = Sphere {
        center: Box::new(Vector {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        }),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractiveness: 1.5,
        }),
    };
    world.push(Box::new(sphere4));
    let sphere5 = Sphere {
        center: Box::new(Vector {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        }),
        radius: -0.45,
        material: Box::new(Dielectric {
            refractiveness: 1.5,
        }),
    };
    world.push(Box::new(sphere5));

    world
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    // consider:
    // let buffer: &[u8] = ...;
    // image::save_buffer(&Path::new("image.png"), buffer, 800, 600, image::RGB(8))

    let mut img = ImageBuffer::new(nx, ny);

    let world = make_scene();
    let look_from = Vector {
        x: 3.0,
        y: 3.0,
        z: 2.0,
    };
    let look_at = Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let view_up = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = (&look_from - &look_at).length();
    let aperture = 2.0;
    let cam = Camera::new(
        &look_from,
        &look_at,
        &view_up,
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    for j in 0..ny {
        for i in 0..nx {
            let mut col = Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
            for _ in 0..ns {
                let u = (i as f64 + rand::random::<f64>()) / nx as f64;
                let v = (j as f64 + rand::random::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                col = col + color(r, &world, 0);
            }
            col = col / ns as f64;

            col = col.gamma_correct();

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
