use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::trace::{Hit, Hittable, Ray};

pub struct MovSphere<M: Scatterable> {
    center_begin: Vec3,
    time_begin: f32,
    center_end: Vec3,
    time_end: f32,
    r: f32,
    mat: M,
}

impl<M: Scatterable> MovSphere<M> {
    pub fn new(
        center_begin: Vec3,
        time_begin: f32,
        center_end: Vec3,
        time_end: f32,
        r: f32,
        mat: M,
    ) -> MovSphere<M> {
        MovSphere {
            center_begin,
            time_begin,
            center_end,
            time_end,
            r,
            mat,
        }
    }

    fn center(&self, time: f32) -> Vec3 {
        self.center_begin
            + ((time - self.time_begin) / (self.time_end / self.time_begin))
                * (self.center_end - self.center_begin)
    }
}

impl<M: Scatterable> Hittable for MovSphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.r * self.r;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }

        let tmp = (-b - disc.sqrt()) / (2.0 * a);
        if tmp < t_max && tmp > t_min {
            let p = r.point_at_param(tmp);
            return Some(Hit::new(
                tmp,
                p,
                (p - self.center(r.time())) / self.r,
                &self.mat,
            ));
        } else {
            let tmp = (-b + disc.sqrt()) / (2.0 * a);
            if tmp < t_max && tmp > t_min {
                let p = r.point_at_param(tmp);
                return Some(Hit::new(
                    tmp,
                    p,
                    (p - self.center(r.time())) / self.r,
                    &self.mat,
                ));
            }
        }

        None
    }
}
