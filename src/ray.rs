use vector::Vector;

#[derive(Debug)]
pub struct Ray {
    a: Vector,
    b: Vector,
}

impl Ray {
    pub fn new(a: &Vector, b: &Vector) -> Ray {
        Ray {
            a: a.clone(),
            b: b.clone(),
        }
    }

    pub fn origin(&self) -> Vector {
        self.a.clone()
    }

    pub fn direction(&self) -> Vector {
        self.b.clone()
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        &self.a + &(t * &self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use vector::Vector;

    #[test]
    fn test_point_at_parameter() {
        let ray = Ray::new(&Vector {
                               x: 0.1,
                               y: 0.1,
                               z: 0.1,
                           },
                           &Vector::new_ones_vector());
        assert_eq!(ray.point_at_parameter(0.5),
                   Vector {
                       x: 0.6,
                       y: 0.6,
                       z: 0.6,
                   });
    }
}
