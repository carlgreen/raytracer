use ray::Ray;
use vector::Vector;

pub trait Hitable {
    fn hit(&self,
           ray: &Ray,
           t_min: f64,
           t_max: f64)
           -> (bool, f64, Vector, Vector, bool, Vector, Ray);
}

pub struct Hitables<'a> {
    pub objects: &'a [&'a Hitable],
}

impl<'a> Hitable for Hitables<'a> {
    fn hit(&self,
           ray: &Ray,
           t_min: f64,
           t_max: f64)
           -> (bool, f64, Vector, Vector, bool, Vector, Ray) {
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
        let mut scatter_ok = false;
        let mut attenuation = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut scattered = Ray::new(&Vector {
                                         x: 0.0,
                                         y: 0.0,
                                         z: 0.0,
                                     },
                                     &Vector {
                                         x: 0.0,
                                         y: 0.0,
                                         z: 0.0,
                                     });
        for obj in self.objects.iter() {
            let (thishit, t, thisp, thisn, thisscatok, thisat, thissc) = obj.hit(ray,
                                                                                 t_min,
                                                                                 closest);
            if thishit {
                hit = true;
                closest = t;
                p = thisp;
                n = thisn;
                scatter_ok = thisscatok;
                attenuation = thisat;
                scattered = thissc;
            }
        }
        (hit, closest, p, n, scatter_ok, attenuation, scattered)
    }
}
