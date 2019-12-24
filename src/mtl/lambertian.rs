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

impl Scatterable<Lambertian> for Lambertian {
    fn scatter(&self, _: &Ray, hit: Hit<Lambertian>) -> Option<(Ray, Vec3)> {
        let target = hit.p() + hit.n() + rand_in_unit_sphere();
        Some((Ray::new(hit.p(), target - hit.p()), self.color))
    }
}
