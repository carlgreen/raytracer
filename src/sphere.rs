use hitable::Hitable;
use material::Material;
use ray::Ray;
use vector::Vector;

pub struct Sphere<'a> {
    pub center: &'a Vector,
    pub radius: f64,
    pub material: &'a Material,
}

// TODO dedupe
fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self,
           ray: &Ray,
           t_min: f64,
           t_max: f64)
           -> (bool, f64, Vector, Vector, bool, Vector, Ray) {
        let oc = &ray.origin() - self.center;
        let a = dot(&ray.direction(), &ray.direction());
        let b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
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
        return (false,
                0.0,
                Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
                Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
                false,
                Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
                Ray::new(&Vector {
                             x: 0.0,
                             y: 0.0,
                             z: 0.0,
                         },
                         &Vector {
                             x: 0.0,
                             y: 0.0,
                             z: 0.0,
                         }));
    }
}

fn scatter(material: &Material, ray: &Ray, p: &Vector, n: &Vector) -> (bool, Vector, Ray) {
    material.scatter(ray, p, n)
}
