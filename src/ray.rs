use vector::Vector;

pub struct Ray {
    a: Vector,
    b: Vector,
}

impl Ray {
    pub fn new(a: &Vector, b: &Vector) -> Ray {
        Ray{
            a: Vector{x: a.x, y: a.y, z: a.z},
            b: Vector{x: b.x, y: b.y, z: b.z},
        }
    }

    pub fn origin(&self) -> Vector {
        Vector{x: self.a.x, y: self.a.y, z: self.a.z}
    }

    pub fn direction(&self) -> Vector {
        Vector{x: self.b.x, y: self.b.y, z: self.b.z}
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        Vector { x: self.a.x + t * self.b.x, y: self.a.y + t * self.b.y, z: self.a.z + t * self.b.z }
    }
}
