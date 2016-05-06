extern crate rand;

use ray::Ray;
use vector::Vector;

pub trait Material {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vector, n: &'a Vector) -> (bool, Vector, Ray);
}

pub struct Lambertian {
    pub albedo: Vector,
}

// TODO dedupe
fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

// TODO dedupe
fn random_in_unit_sphere() -> Vector {
    loop {
        let p = &(2.0 *
                  &Vector {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }) -
                &Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
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

// TODO dedupe
fn unit_vector(vector: &Vector) -> Vector {
    return vector / vector.length();
}

pub struct Metal {
    pub albedo: Vector,
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
    return v - &(2.0 * dot(v, n) * n);
}

impl Material for Metal {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vector, n: &'a Vector) -> (bool, Vector, Ray) {
        let reflected = reflect(&unit_vector(&ray.direction()), n);
        let scattered = Ray::new(p, &(&reflected + &(self.fuzz * &random_in_unit_sphere())));
        let attenuation = self.albedo.clone();
        let ok = dot(&scattered.direction(), n) > 0.0;
        (ok, attenuation, scattered)
    }
}
