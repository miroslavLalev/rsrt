use crate::math::Vec3;
use crate::trace::{Hit, Ray};
use crate::utils::rng::{rand_in_unit_sphere, uniform_in_range};

use super::utils as mtl_utils;
use super::Scatterable;

pub struct Dielectric {
    rfn_ind: f32,
}

impl Dielectric {
    pub fn new(rfn_ind: f32) -> Dielectric {
        Dielectric { rfn_ind }
    }
}

impl Scatterable<Dielectric> for Dielectric {
    fn scatter(&self, r: &Ray, hit: Hit<Dielectric>) -> Option<(Ray, Vec3)> {
        let reflected = mtl_utils::reflect(r.direction(), hit.n());

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

        let (refracted, reflect_prob) = match mtl_utils::refract(r.direction(), out_norm, ni_nt) {
            Some(refracted) => (Some(refracted), mtl_utils::schlick(self.rfn_ind, cos)),
            None => (None, 1.0),
        };

        if uniform_in_range(0.0, 1.0) < reflect_prob {
            Some((Ray::new(hit.p(), refracted?), Vec3(1.0, 1.0, 1.0)))
        } else {
            Some((Ray::new(hit.p(), reflected), Vec3(1.0, 1.0, 1.0)))
        }
    }
}
