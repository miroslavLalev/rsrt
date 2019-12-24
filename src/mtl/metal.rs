use crate::math::Vec3;
use crate::trace::{Hit, Ray};
use crate::utils::rng::rand_in_unit_sphere;

use super::utils as mtl_utils;
use super::Scatterable;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: f32::min(fuzz, 1.0),
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        let reflected = mtl_utils::reflect(r.direction().as_unit(), hit.n());
        let scattered = Ray::new(hit.p(), reflected + self.fuzz * rand_in_unit_sphere());
        if scattered.direction().dot(hit.n()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
