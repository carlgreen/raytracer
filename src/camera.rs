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
    pub fn new(from: &Vector, at: &Vector, up: &Vector, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (from - at).unit_vector();
        let u = Vector::cross(up, &w).unit_vector();
        let v = Vector::cross(&w, &u);
        Camera {
            lower_left_corner: &(&(from - &(half_width * &u)) + &(half_height * &v)) - &w,
            horizontal: 2.0 * half_width * &u,
            vertical: -2.0 * half_height * &v,
            origin: from.clone(),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin,
                 &(&(&(&self.lower_left_corner + &(u * &self.horizontal)) +
                     &(v * &self.vertical)) - &self.origin))
    }
}
