use crate::math::Vec3;
use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};

pub struct Translate<H: Hittable> {
    hittable: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(hittable: H, offset: Vec3) -> Translate<H> {
        Translate { hittable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(hit) = self.hittable.hit(&moved_ray, t_min, t_max) {
            return Some(Hit::new(
                hit.t(),
                hit.p() + self.offset,
                hit.n(),
                hit.mat_ref(),
                hit.u(),
                hit.v(),
            ));
        }

        None
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> AABB {
        let bbox = self.hittable.bounding_box(t_min, t_max);
        AABB::new(
            bbox.min() + self.offset,
            bbox.max() + self.offset,
        )
    }
}
