use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::tex::Wrappable;
use crate::trace::{Hit, Ray};

pub struct LightDiffuse<W: Wrappable> {
    emit: W,
}

impl<W: Wrappable> LightDiffuse<W> {
    pub fn new(emit: W) -> LightDiffuse<W> {
        LightDiffuse { emit }
    }
}

impl<W: Wrappable> Scatterable for LightDiffuse<W> {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
