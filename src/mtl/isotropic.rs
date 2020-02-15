use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::tex::Wrappable;
use crate::trace::{Hit, Ray};
use crate::utils::rng::rand_in_unit_sphere;

pub struct Isotropic<W: Wrappable> {
    albedo: W,
}

impl<W: Wrappable> Isotropic<W> {
    pub fn new(albedo: W) -> Isotropic<W> {
        Isotropic { albedo }
    }
}

impl<W: Wrappable> Scatterable for Isotropic<W> {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        Some((
            Ray::new(hit.p(), rand_in_unit_sphere(), r.time()),
            self.albedo.value(hit.u(), hit.v(), hit.p()),
        ))
    }
}
