extern crate image;
extern crate rand;

mod camera;
mod color;
mod hitable;
mod ray;
mod sphere;
mod vector;

use std::f64;
use std::fs::File;
use std::path::Path;
use image::{GenericImage, ImageBuffer};

use camera::Camera;
use color::Color;
use hitable::Hitable;
use hitable::Hitables;
use ray::Ray;
use sphere::Sphere;
use vector::Vector;

fn unit_vector(vector: &Vector) -> Vector {
    return vector / vector.length();
}

#[cfg(test)]
mod tests {
    use vector::Vector;
    use super::unit_vector;

    #[test]
    fn test_already_unit_vector() {
        assert_eq!(Vector {
                       x: 1.0,
                       y: 0.0,
                       z: 0.0,
                   },
                   unit_vector(&Vector {
                       x: 1.0,
                       y: 0.0,
                       z: 0.0,
                   }));
    }

    #[test]
    fn test_non_unit_vector() {
        let got = unit_vector(&Vector {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        });
        assert!(got.y > 0.707 && got.y < 0.708);
        assert!(got.z > 0.707 && got.z < 0.708);
    }
}

// TODO dedupe
fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn random_in_unit_sphere() -> Vector {
    loop {
        let p = &(2.0 *
                  &Vector {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }) -
                &Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

fn color(ray: Ray, hitable: &Hitable) -> Color {
    let (hit, _, p, n) = hitable.hit(&ray, 0.0, f64::MAX);
    if hit {
        let target = &(&p + &n) + &random_in_unit_sphere();
        return 0.5 * color(Ray::new(&p, &(&target - &p)), hitable);
    }
    let unit_direction = unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    let v = &((1.0 - t) *
              &Vector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }) +
            &(t *
              &Vector {
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

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    // consider:
    // let buffer: &[u8] = ...;
    // image::save_buffer(&Path::new("image.png"), buffer, 800, 600, image::RGB(8))

    let mut img = ImageBuffer::new(nx, ny);

    let lower_left_corner = Vector {
        x: -2.0,
        y: 1.0,
        z: -1.0,
    };
    let horizontal = Vector {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vector {
        x: 0.0,
        y: -2.0,
        z: 0.0,
    };
    let origin = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let sphere1 = Sphere {
        center: &Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: &Vector {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    };
    let world = Hitables { objects: &[&sphere1, &sphere2] };
    let cam = Camera {
        lower_left_corner: lower_left_corner,
        horizontal: horizontal,
        vertical: vertical,
        origin: origin,
    };

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
                col = col + color(r, &world);
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
