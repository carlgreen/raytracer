use ray::Ray;
use vector::Vector;

pub trait Hitable {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> (bool, f64, Vector, Vector, bool, Vector, Ray);
}

impl<'a> Hitable for Vec<&'a Hitable> {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> (bool, f64, Vector, Vector, bool, Vector, Ray) {
        let mut hit = false;
        let mut p = Vector::new_zero_vector();
        let mut n = Vector::new_zero_vector();
        let mut closest = t_max;
        let mut scatter_ok = false;
        let mut attenuation = Vector::new_zero_vector();
        let mut scattered = Ray::new(&Vector::new_zero_vector(), &Vector::new_zero_vector());
        for obj in self.iter() {
            let (thishit, t, thisp, thisn, thisscatok, thisat, thissc) =
                obj.hit(ray, t_min, closest);
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
