use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::{surrounding_box, AABB};
use crate::trace::Ray;

pub struct Hit<'a> {
    t: f32,  // from p(t) = A + t * B
    p: Vec3, // point_at_param(t)
    n: Vec3, // normal
    mat: &'a dyn Scatterable,

    // for texture mapping
    u: f32,
    v: f32,
}

impl<'a> Hit<'a> {
    pub fn new(t: f32, p: Vec3, n: Vec3, mat: &'a dyn Scatterable, u: f32, v: f32) -> Hit {
        Hit { t, p, n, mat, u, v }
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

    pub fn u(&self) -> f32 {
        self.u
    }

    pub fn v(&self) -> f32 {
        self.v
    }

    pub fn mat_ref(&self) -> &'a dyn Scatterable {
        self.mat
    }

    pub fn scatter(self, r: &Ray) -> Option<(Ray, Vec3)> {
        self.mat.scatter(r, self)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB>;
}

pub struct HitVec {
    elements: Vec<Box<dyn Hittable>>,
}

impl HitVec {
    pub fn new(elements: Vec<Box<dyn Hittable>>) -> HitVec {
        HitVec { elements }
    }
}

impl Hittable for HitVec {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_t = t_max;
        let mut last_hit = None;
        for elem in &self.elements {
            if let Some(hit) = elem.hit(r, t_min, closest_t) {
                closest_t = hit.t;
                last_hit = Some(hit);
            }
        }
        last_hit
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        let mut iter = self.elements.iter();
        let first_item = iter.next()?;
        let mut tmp_box = first_item.bounding_box(t_min, t_max)?;

        for item in iter {
            tmp_box = surrounding_box(item.bounding_box(t_min, t_max)?, tmp_box);
        }
        Some(tmp_box)
    }
}

// &HitVec can be shared between threads, as it can't be mutated after construction.
unsafe impl Send for HitVec {}
unsafe impl Sync for HitVec {}
