use ray::Ray;
use vector::Vector;

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, Vector);
}
