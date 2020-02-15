use super::Scatterable;
use crate::math::Vec3;
use crate::tex::Wrappable;
use crate::trace::{Hit, Ray};
use crate::utils::rng::rand_in_unit_sphere;

#[derive(Clone)]
pub struct Lambertian<W: Wrappable> {
    albedo: W,
}

impl<W: Wrappable> Lambertian<W> {
    pub fn new(albedo: W) -> Lambertian<W> {
        Lambertian { albedo }
    }
}

impl<W: Wrappable> Scatterable for Lambertian<W> {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let target = hit.p() + hit.n() + rand_in_unit_sphere();
        Some((
            Ray::new(hit.p(), target - hit.p(), r.time()),
            self.albedo.value(hit.u(), hit.v(), hit.p()),
        ))
    }
}
