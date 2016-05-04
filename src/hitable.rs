use ray::Ray;
use vector::Vector;

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, f64, Vector, Vector);
}

pub struct Hitables<'a> {
    pub objects: &'a [&'a Hitable],
}

impl<'a> Hitable for Hitables<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, f64, Vector, Vector) {
        let mut hit = false;
        let mut p = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut n = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut closest = t_max;
        for obj in self.objects.iter() {
            let (thishit, t, thisp, thisn) = obj.hit(ray, t_min, closest);
            if thishit {
                hit = true;
                closest = t;
                p = thisp;
                n = thisn;
            }
        }
        (hit, closest, p, n)
    }
}
