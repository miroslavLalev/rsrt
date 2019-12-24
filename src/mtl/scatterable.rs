use crate::math::Vec3;
use crate::trace::{Hit, Ray};

pub trait Scatterable<M: Scatterable<M>> {
    fn scatter(&self, r: &Ray, hit: Hit<M>) -> Option<(Ray, Vec3)>;
}
