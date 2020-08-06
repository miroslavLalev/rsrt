use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};

pub struct FlipNormals<H: Hittable> {
    hittable: H,
}

impl<H: Hittable> FlipNormals<H> {
    pub fn new(hittable: H) -> FlipNormals<H> {
        FlipNormals { hittable }
    }
}

impl<H: Hittable> Hittable for FlipNormals<H> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if let Some(hit) = self.hittable.hit(r, t_min, t_max) {
            return Some(Hit::new(
                hit.t(),
                hit.p(),
                -hit.n(),
                hit.mat_ref(),
                hit.u(),
                hit.v(),
            ));
        }

        None
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> AABB {
        self.hittable.bounding_box(t_min, t_max)
    }
}
