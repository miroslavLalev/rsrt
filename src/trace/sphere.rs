use super::super::math::Vec3;
use super::Scatterable;
use super::Ray;
use super::{Hit, Hittable};

pub struct Sphere {
    center: Vec3,
    r: f32,
}

impl Sphere {
    pub fn new(center: Vec3, r: f32) -> Sphere {
        Sphere { center, r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
            return Some(Hit::new(tmp, p, (p - self.center) / self.r));
        } else {
            let tmp = (-b + disc.sqrt()) / (2.0 * a);
            if tmp < t_max && tmp > t_min {
                let p = r.point_at_param(tmp);
                return Some(Hit::new(tmp, p, (p - self.center) / self.r));
            }
        }

        None
    }
}
