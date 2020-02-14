use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::tex::Wrappable;
use crate::trace::{Hit, Ray};

pub struct LightDiffuse {
    emit: Box<dyn Wrappable>,
}

impl LightDiffuse {
    pub fn new(emit: Box<dyn Wrappable>) -> LightDiffuse {
        LightDiffuse { emit }
    }
}

impl Scatterable for LightDiffuse {
    fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
