use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::transform::FlipNormals;
use crate::obj::{XYRect, XZRect, YZRect, AABB};
use crate::trace::{Hit, HitVec, Hittable, Ray};

pub struct RectBox {
    p_min: Vec3,
    p_max: Vec3,

    sides: HitVec,
}

impl RectBox {
    pub fn new<M: Scatterable + Clone + 'static>(p_min: Vec3, p_max: Vec3, mat: M) -> RectBox {
        let mut sides: Vec<Box<dyn Hittable>> = Vec::with_capacity(6);
        sides.push(Box::new(XYRect::new(
            p_min.0,
            p_min.1,
            p_max.0,
            p_max.1,
            p_max.2,
            mat.clone(),
        )));
        sides.push(Box::new(FlipNormals::new(XYRect::new(
            p_min.0,
            p_min.1,
            p_max.0,
            p_max.1,
            p_min.2,
            mat.clone(),
        ))));
        sides.push(Box::new(XZRect::new(
            p_min.0,
            p_min.2,
            p_max.0,
            p_max.2,
            p_max.1,
            mat.clone(),
        )));
        sides.push(Box::new(FlipNormals::new(XZRect::new(
            p_min.0,
            p_min.2,
            p_max.0,
            p_max.2,
            p_min.1,
            mat.clone(),
        ))));
        sides.push(Box::new(YZRect::new(
            p_min.1,
            p_min.2,
            p_max.1,
            p_max.2,
            p_max.0,
            mat.clone(),
        )));
        sides.push(Box::new(FlipNormals::new(YZRect::new(
            p_min.1,
            p_min.2,
            p_max.1,
            p_max.2,
            p_min.0,
            mat.clone(),
        ))));

        RectBox {
            p_min,
            p_max,
            sides: HitVec::new(sides),
        }
    }
}

impl Hittable for RectBox {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _: f32, _: f32) -> AABB {
        AABB::new(self.p_min, self.p_max)
    }
}
