use crate::math::Vec3;
use crate::trace::{Hit, Ray};

pub trait Scatterable {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)>;
}
