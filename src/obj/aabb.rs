use crate::math::Vec3;
use crate::trace::Ray;

// AABB represents axis-aligned boundig box
#[derive(Copy, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let (t0, t1) = hit_axis(self.min.0, self.max.0, r.origin().0, r.direction().0);
        let t_min = t_min.max(t0);
        let t_max = t_max.min(t1);
        if t_min < t_max {
            return false;
        }

        let (t0, t1) = hit_axis(self.min.1, self.max.1, r.origin().1, r.direction().1);
        let t_min = t_min.max(t0);
        let t_max = t_max.min(t1);
        if t_min < t_max {
            return false;
        }

        let (t0, t1) = hit_axis(self.min.2, self.max.2, r.origin().2, r.direction().2);
        let t_min = t_min.max(t0);
        let t_max = t_max.min(t1);
        if t_min < t_max {
            return false;
        }

        true
    }
}

#[inline]
fn hit_axis(min_axis: f32, max_axis: f32, r_orig: f32, r_dir: f32) -> (f32, f32) {
    let t0 = ((min_axis - r_orig) / r_dir).min((max_axis - r_orig) / r_dir);
    let t1 = ((min_axis - r_orig) / r_dir).max((max_axis - r_orig) / r_dir);

    (t0, t1)
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let min = Vec3(
        box0.min.0.min(box1.min.0),
        box0.min.1.min(box1.min.1),
        box0.min.2.min(box1.min.2),
    );
    let max = Vec3(
        box0.min.0.max(box1.min.0),
        box0.min.1.max(box1.min.1),
        box0.min.2.max(box1.min.2),
    );

    AABB { min, max }
}
