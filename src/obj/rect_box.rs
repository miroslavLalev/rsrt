use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::{XYRect, XZRect, YZRect, AABB};
use crate::trace::{Hit, HitVec, Hittable, Ray};
use std::rc::Rc;
use crate::obj::transform::FlipNormals;

pub struct RectBox<M: Scatterable> {
    p_min: Vec3,
    p_max: Vec3,
    mat: M,

    sides: HitVec,
}

impl<M: Scatterable + Clone + 'static> RectBox<M> {
    pub fn new(p_min: Vec3, p_max: Vec3, mat: M) -> RectBox<M> {
        let mut sides: Vec<Box<dyn Hittable>> = Vec::with_capacity(6);
        sides.push(Box::new(XYRect::new(
            p_min.0,
            p_min.1,
            p_max.0,
            p_max.1,
            p_max.2,
            mat.clone(),
        )));
        sides.push(Box::new(FlipNormals::new(Box::new(XYRect::new(
            p_min.0,
            p_min.1,
            p_max.0,
            p_max.1,
            p_min.2,
            mat.clone(),
        )))));
        sides.push(Box::new(XZRect::new(
            p_min.0,
            p_min.2,
            p_max.0,
            p_max.2,
            p_max.1,
            mat.clone(),
        )));
        sides.push(Box::new(FlipNormals::new(Box::new(XZRect::new(
            p_min.0,
            p_min.2,
            p_max.0,
            p_max.2,
            p_min.1,
            mat.clone(),
        )))));
        sides.push(Box::new(YZRect::new(
            p_min.1,
            p_min.2,
            p_max.1,
            p_max.2,
            p_max.0,
            mat.clone(),
        )));
        sides.push(Box::new(FlipNormals::new(Box::new(YZRect::new(
            p_min.1,
            p_min.2,
            p_max.1,
            p_max.2,
            p_min.0,
            mat.clone(),
        )))));

        RectBox {
            p_min,
            p_max,
            mat,
            sides: HitVec::new(sides),
        }
    }
}

impl<M: Scatterable> Hittable for RectBox<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(AABB::new(self.p_min, self.p_max))
    }
}
