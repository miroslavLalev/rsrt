use super::super::math::Vec3;
use super::Ray;
use super::Scatterable;

pub struct Hit<'a, M: Scatterable<M>> {
    t: f32,  // from p(t) = A + t * B
    p: Vec3, // point_at_param(t)
    n: Vec3, // normal
    mat: &'a M,
}

impl<'a, M: Scatterable<M>> Hit<'a, M> {
    pub fn new(t: f32, p: Vec3, n: Vec3, mat: &'a M) -> Hit<M> {
        Hit { t, p, n, mat }
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

    pub fn scatter(self, r: &Ray) -> Option<(Ray, Vec3)> {
        self.mat.scatter(r, self)
    }
}

pub trait Hittable<M: Scatterable<M>> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit<M>>;
}
