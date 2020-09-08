use crate::math::Vec3;
use crate::trace::{Hit, Ray};
use crate::tex::Wrappable;
use crate::utils::rng::uniform_in_range;

use super::utils as mtl_utils;
use super::Scatterable;

pub struct Dielectric<W: Wrappable> {
    rfn_ind: f32,
    albedo: W,
}

impl <W: Wrappable> Dielectric<W> {
    pub fn new(rfn_ind: f32, albedo: W) -> Dielectric<W> {
        Dielectric { rfn_ind, albedo }
    }
}

impl <W: Wrappable> Scatterable for Dielectric<W> {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let reflected = mtl_utils::reflect(r.direction().as_unit(), hit.n());

        let (out_norm, ni_nt, cos) = if r.direction().dot(hit.n()) > 0.0 {
            (
                -hit.n(),
                self.rfn_ind,
                self.rfn_ind * r.direction().dot(hit.n()) / r.direction().len(),
            )
        } else {
            (
                hit.n(),
                1.0 / self.rfn_ind,
                -r.direction().dot(hit.n()) / r.direction().len(),
            )
        };

        let (refracted, reflect_prob) = match mtl_utils::refract(r.direction().as_unit(), out_norm, ni_nt) {
            Some(refracted) => (Some(refracted), mtl_utils::schlick(self.rfn_ind, cos)),
            None => (None, 1.0),
        };

        if uniform_in_range(0.0, 1.0) < reflect_prob {
            Some((Ray::new(hit.p(), reflected, r.time()), self.albedo.value(hit.u(), hit.v(), hit.p())))
        } else {
            Some((Ray::new(hit.p(), refracted?, r.time()), self.albedo.value(hit.u(), hit.v(), hit.p())))
        }
    }
}
