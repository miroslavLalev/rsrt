use super::Scatterable;
use crate::math::Vec3;
use crate::tex::Wrappable;
use crate::trace::{Hit, Ray};
use crate::utils::rng::rand_in_unit_sphere;

pub struct Lambertian {
    albedo: Box<dyn Wrappable>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Wrappable>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let target = hit.p() + hit.n() + rand_in_unit_sphere();
        Some((
            Ray::new(hit.p(), target - hit.p(), r.time()),
            self.albedo.value(hit.u(), hit.v(), hit.p()),
        ))
    }
}
