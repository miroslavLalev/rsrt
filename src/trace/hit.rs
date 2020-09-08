use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::{surrounding_box, AABB};
use crate::trace::Ray;

/// Hit represents a point that has been hit by a ray.
/// It contains the following fields:
///     t - the ray function dimension where the hit occured
///     p - the ray function value vector where the hit occured
///     n - the hit surface normal
///     mat - reference for the material that was hit
/// TODO: u, v
pub struct Hit<'a> {
    t: f32,
    p: Vec3,
    n: Vec3,
    mat: &'a dyn Scatterable,

    // for texture mapping
    u: f32,
    v: f32,
}

impl<'a> Hit<'a> {
    /// Returns a new hit for the given parameters.
    pub fn new(t: f32, p: Vec3, n: Vec3, mat: &'a dyn Scatterable, u: f32, v: f32) -> Hit {
        Hit { t, p, n, mat, u, v }
    }

    /// Accessor for hit.t.
    pub fn t(&self) -> f32 {
        self.t
    }

    /// Accessor for hit.p.
    pub fn p(&self) -> Vec3 {
        self.p
    }

    /// Accessor for hit.n.
    pub fn n(&self) -> Vec3 {
        self.n
    }

    /// Accessor for hit.u.
    pub fn u(&self) -> f32 {
        self.u
    }

    /// Accessor for hit.v.
    pub fn v(&self) -> f32 {
        self.v
    }

    /// Accessor for hit.mat.
    pub fn mat_ref(&self) -> &'a dyn Scatterable {
        self.mat
    }

    /// Scatter is a proxy function for the hit material.
    /// It consumes the hit.
    pub fn scatter(self, r: &Ray) -> Option<(Ray, Vec3)> {
        self.mat.scatter(r, self)
    }
}

/// Hittable is a trait for objects that could be hit.
pub trait Hittable {
    /// Hit returns whether the passed ray hits the object within the given
    /// limits for t.
    /// Returns None if no hit occurs.
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    /// Returns the bounding box for the object.
    fn bounding_box(&self, t_min: f32, t_max: f32) -> AABB;
}

/// HitVec is a structure for holding multiple hittable objects.
pub struct HitVec {
    elements: Vec<Box<dyn Hittable>>,
}

impl HitVec {
    /// Creates a new HitVec for the given hittables.
    pub fn new(elements: Vec<Box<dyn Hittable>>) -> HitVec {
        HitVec { elements }
    }
}

impl Hittable for HitVec {
    /// Hit implements Hittable and returns the closest hit from
    /// the elements of HitVec.
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

    /// Returns the surrounding box of all hittable elements.
    fn bounding_box(&self, t_min: f32, t_max: f32) -> AABB {
        self.elements.iter().fold(
            AABB::new_hidden(),
            |res, cur| {
                surrounding_box(res, cur.bounding_box(t_min, t_max))
            }
        )
    }
}

// &HitVec can be shared between threads, as it can't be mutated after construction.
unsafe impl Send for HitVec {}
unsafe impl Sync for HitVec {}
