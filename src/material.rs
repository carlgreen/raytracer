extern crate rand;

use ray::Ray;
use vector::Vector;
use vector::dot;

pub trait Material {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vector, n: &'a Vector) -> (bool, Vector, Ray);
}

pub struct Lambertian {
    pub albedo: Vector,
}

fn random_in_unit_sphere() -> Vector {
    loop {
        let p = &(2.0 *
                  &Vector {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }) - &Vector::new_ones_vector();
        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&self, _: &Ray, p: &'a Vector, n: &'a Vector) -> (bool, Vector, Ray) {
        let target = &(p + n) + &random_in_unit_sphere();
        let scattered = Ray::new(p, &(&target - p));
        let attenuation = self.albedo.clone();
        (true, attenuation, scattered)
    }
}

pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vector, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal {
            albedo: albedo.clone(),
            fuzz: f,
        }
    }
}

fn reflect(v: &Vector, n: &Vector) -> Vector {
    v - &(2.0 * dot(v, n) * n)
}

impl Material for Metal {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vector, n: &'a Vector) -> (bool, Vector, Ray) {
        let reflected = reflect(&ray.direction().unit_vector(), n);
        let scattered = Ray::new(p, &(&reflected + &(self.fuzz * &random_in_unit_sphere())));
        let attenuation = self.albedo.clone();
        let ok = dot(&scattered.direction(), n) > 0.0;
        (ok, attenuation, scattered)
    }
}

pub struct Dielectric {
    pub refractiveness: f64,
}

fn refract(v: &Vector, n: &Vector, ni_over_nt: f64) -> (bool, Vector) {
    let uv = v.unit_vector();
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refraction = &(ni_over_nt * &(v - &(dt * n))) - &(discriminant.sqrt() * n);
        return (true, refraction);
    }
    (false, Vector::new_zero_vector())
}

fn schlick(cosine: f64, refractiveness: f64) -> f64 {
    let r0 = ((1.0 - refractiveness) / (1.0 + refractiveness)).powi(2);
    r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
}

impl Material for Dielectric {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vector, n: &'a Vector) -> (bool, Vector, Ray) {
        let reflection = reflect(&ray.direction(), n);
        let attenuation = Vector::new_ones_vector();
        let (outward_normal, ni_over_nt, cosine) = if dot(&ray.direction(), n) > 0.0 {
            (-n,
             self.refractiveness,
             self.refractiveness * dot(&ray.direction(), n) / ray.direction().length())
        } else {
            (n.clone(),
             1.0 / self.refractiveness,
             -dot(&ray.direction(), n) / ray.direction().length())
        };
        let (refracted, refraction) = refract(&ray.direction(), &outward_normal, ni_over_nt);
        let reflect_prob = if refracted {
            schlick(cosine, self.refractiveness)
        } else {
            1.0
        };
        let scattered = if rand::random::<f64>() < reflect_prob {
            Ray::new(p, &reflection)
        } else {
            Ray::new(p, &refraction)
        };
        (true, attenuation, scattered)
    }
}
