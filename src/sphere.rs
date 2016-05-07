use hitable::Hitable;
use material::Material;
use ray::Ray;
use vector::Vector;

pub struct Sphere<'a> {
    pub center: &'a Vector,
    pub radius: f64,
    pub material: &'a Material,
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self,
           ray: &Ray,
           t_min: f64,
           t_max: f64)
           -> (bool, f64, Vector, Vector, bool, Vector, Ray) {
        let oc = &ray.origin() - self.center;
        let a = Vector::dot(&ray.direction(), &ray.direction());
        let b = Vector::dot(&oc, &ray.direction());
        let c = Vector::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = &(&p - self.center) / self.radius;
                let (scatter_ok, attenuation, scattered) = scatter(self.material, ray, &p, &normal);
                return (true, temp, p, normal, scatter_ok, attenuation, scattered);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = &(&p - self.center) / self.radius;
                let (scatter_ok, attenuation, scattered) = scatter(self.material, ray, &p, &normal);
                return (true, temp, p, normal, scatter_ok, attenuation, scattered);
            }
        }
        (false,
         0.0,
         Vector::new_zero_vector(),
         Vector::new_zero_vector(),
         false,
         Vector::new_zero_vector(),
         Ray::new(&Vector::new_zero_vector(), &Vector::new_zero_vector()))
    }
}

fn scatter(material: &Material, ray: &Ray, p: &Vector, n: &Vector) -> (bool, Vector, Ray) {
    material.scatter(ray, p, n)
}
