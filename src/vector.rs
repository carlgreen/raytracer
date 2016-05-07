use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new_zero_vector() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_ones_vector() -> Vector {
        Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit_vector(&self) -> Vector {
        self / self.length()
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

impl<'a> Add<&'a Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;

    fn div(self, div: f64) -> Vector {
        Vector {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

impl<'a> Mul<&'a Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: &Vector) -> Vector {
        Vector {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl<'a> Neg for &'a Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a, 'b> Sub<&'a Vector> for &'b Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn test_already_unit_vector() {
        assert_eq!(Vector {
                       x: 1.0,
                       y: 0.0,
                       z: 0.0,
                   },
                   Vector {
                       x: 1.0,
                       y: 0.0,
                       z: 0.0,
                   }
                   .unit_vector());
    }

    #[test]
    fn test_non_unit_vector() {
        let got = &Vector {
                       x: 0.0,
                       y: 1.0,
                       z: 1.0,
                   }
                   .unit_vector();
        assert!(got.y > 0.707 && got.y < 0.708);
        assert!(got.z > 0.707 && got.z < 0.708);
    }
}
