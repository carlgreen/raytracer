use vector::Vector;

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
        Vector {
            x: self.a.x + t * self.b.x,
            y: self.a.y + t * self.b.y,
            z: self.a.z + t * self.b.z,
        }
    }
}
