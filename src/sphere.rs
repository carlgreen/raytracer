use hitable::Hitable;
use ray::Ray;
use vector::Vector;

pub struct Sphere<'a> {
    pub center: &'a Vector,
    pub radius: f64,
}

// TODO dedupe
fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, Vector) {
        let oc = &ray.origin() - self.center;
        let a = dot(&ray.direction(), &ray.direction());
        let b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a   ;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = &(&p - self.center) / self.radius;
                return (true, normal);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = &(&p - self.center) / self.radius;
                return (true, normal);
            }
        }
        return (false, Vector{x: 0.0, y: 0.0, z: 0.0});

    }
}
