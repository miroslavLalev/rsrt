use super::Ray;
use super::super::math::Vec3;

pub trait Scatterable {
    fn scatter(&self, r: &Ray) -> Option<Ray>;
}

pub enum Material {
    Lambertian(Vec3),
}

impl Scatterable for Material {
    fn scatter(&self, r: &Ray) -> Option<Ray> {
        Some(r.clone())
    }
}