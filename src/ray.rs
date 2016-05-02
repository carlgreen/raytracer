use vector::Vector;

pub struct Ray<'a> {
    pub a: &'a Vector,
    pub b: &'a Vector,
}

impl<'a> Ray<'a> {
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
