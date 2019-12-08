use rand::Rng;

use super::super::math::Vec3;
use super::Ray;
use crate::trace::Hit;

pub trait Scatterable<M: Scatterable<M>> {
    fn scatter(&self, r: &Ray, hit: Hit<M>) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    color: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { color }
    }

    fn rand_float() -> f32 {
        rand::thread_rng().gen_range(0.0, 1.0)
    }

    fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let v =
                2.0 * Vec3(
                    Lambertian::rand_float(),
                    Lambertian::rand_float(),
                    Lambertian::rand_float(),
                ) - Vec3(1.0, 1.0, 1.0);
            if v.sq_len() >= 1.0 {
                break v;
            }
        }
    }
}

impl Scatterable<Lambertian> for Lambertian {
    fn scatter(&self, _: &Ray, hit: Hit<Lambertian>) -> Option<(Ray, Vec3)> {
        let target = hit.p() + hit.n() + Lambertian::rand_in_unit_sphere();
        Some((Ray::new(hit.p(), target - hit.p()), self.color))
    }
}
