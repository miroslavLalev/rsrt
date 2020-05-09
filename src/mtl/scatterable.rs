use crate::math::Vec3;
use crate::trace::{Hit, Ray};

pub trait Scatterable {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)>;

    fn emitted(&self, _: f32, _: f32, _: Vec3) -> Vec3 {
        // Black (non-emitting) by default
        Vec3(0.0, 0.0, 0.0)
    }
}
