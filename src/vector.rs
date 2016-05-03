use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn length(&self) -> f64 {
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
