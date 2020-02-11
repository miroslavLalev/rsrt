use super::Scatterable;
use crate::math::Vec3;
use crate::trace::{Hit, Ray};
use crate::utils::rng::rand_in_unit_sphere;

pub struct Lambertian {
    color: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { color }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let target = hit.p() + hit.n() + rand_in_unit_sphere();
        Some((Ray::new(hit.p(), target - hit.p(), r.time()), self.color))
    }
}
