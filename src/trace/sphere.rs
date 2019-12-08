use super::super::math::Vec3;
use super::Ray;
use super::Scatterable;
use super::{Hit, Hittable};

pub struct Sphere<M: Scatterable<M>> {
    center: Vec3,
    r: f32,
    mat: M,
}

impl<M: Scatterable<M>> Sphere<M> {
    pub fn new(center: Vec3, r: f32, mat: M) -> Sphere<M> {
        Sphere { center, r, mat }
    }
}

impl<M: Scatterable<M>> Hittable<M> for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit<M>> {
        let oc = r.origin() - self.center;
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
            return Some(Hit::new(tmp, p, (p - self.center) / self.r, &self.mat));
        } else {
            let tmp = (-b + disc.sqrt()) / (2.0 * a);
            if tmp < t_max && tmp > t_min {
                let p = r.point_at_param(tmp);
                return Some(Hit::new(tmp, p, (p - self.center) / self.r, &self.mat));
            }
        }

        None
    }
}
