use super::super::math::Vec3;
use super::Ray;

pub struct Hit {
    t: f32,  // from p(t) = A + t * B
    p: Vec3, // point_at_param(t)
    n: Vec3, // normal
}

impl Hit {
    pub fn new(t: f32, p: Vec3, n: Vec3) -> Hit {
        Hit { t, p, n }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn n(&self) -> Vec3 {
        self.n
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
