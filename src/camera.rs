use std::f64;

use ray::Ray;
use vector::Vector;

pub struct Camera {
    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub origin: Vector,
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: Vector {
                x: -half_width,
                y: half_height,
                z: -1.0,
            },
            horizontal: Vector {
                x: 2.0 * half_width,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vector {
                x: 0.0,
                y: 2.0 * -half_height,
                z: 0.0,
            },
            origin: Vector::new_zero_vector(),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin,
                 &(&(&self.lower_left_corner + &(u * &self.horizontal)) + &(v * &self.vertical)))
    }
}
