use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};

pub struct Rect<M: Scatterable> {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,

    // z = k => t = (k - a) / b
    k: f32,

    mat: M,
}

impl<M: Scatterable> Rect<M> {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32, k: f32, mat: M) -> Rect<M> {
        Rect {
            x0,
            y0,
            x1,
            y1,
            k,
            mat,
        }
    }
}

impl<M: Scatterable> Hittable for Rect<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin().2) / r.direction().2;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().0 + t * r.direction().0;
        let y = r.origin().1 + t * r.direction().1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(Hit::new(
            t,
            r.point_at_param(t),
            Vec3(0.0, 0.0, 1.0),
            &self.mat,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3(self.x0, self.y0, self.k - 0.0001),
            Vec3(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
