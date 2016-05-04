use ray::Ray;
use vector::Vector;

pub struct Camera {
    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
    pub origin: Vector,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            lower_left_corner: Vector {
                x: -2.0,
                y: 1.0,
                z: -1.0,
            },
            horizontal: Vector {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vector {
                x: 0.0,
                y: -2.0,
                z: 0.0,
            },
            origin: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin,
                 &(&(&self.lower_left_corner + &(u * &self.horizontal)) + &(v * &self.vertical)))
    }
}
