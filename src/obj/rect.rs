use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};

const EPS: f32 = 0.0001;

pub struct XYRect<M: Scatterable> {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,

    // z = k => t = (k - a) / b
    k: f32,

    mat: M,
}

impl<M: Scatterable> XYRect<M> {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32, k: f32, mat: M) -> XYRect<M> {
        XYRect {
            x0,
            y0,
            x1,
            y1,
            k,
            mat,
        }
    }
}

impl<M: Scatterable> Hittable for XYRect<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin().2) / r.direction().2;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().0 + t * r.direction().0;
        let y = r.origin().1 + t * r.direction().1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(Hit::new(
            t,
            r.point_at_param(t),
            Vec3(0.0, 0.0, 1.0),
            &self.mat,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3(self.x0, self.y0, self.k - EPS),
            Vec3(self.x1, self.y1, self.k + EPS),
        ))
    }
}

pub struct XZRect<M: Scatterable> {
    x0: f32,
    z0: f32,
    x1: f32,
    z1: f32,

    // y = k => t = (k - a) / b
    k: f32,

    mat: M,
}

impl<M: Scatterable> XZRect<M> {
    pub fn new(x0: f32, z0: f32, x1: f32, z1: f32, k: f32, mat: M) -> XZRect<M> {
        XZRect {
            x0,
            z0,
            x1,
            z1,
            k,
            mat,
        }
    }
}

impl<M: Scatterable> Hittable for XZRect<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin().1) / r.direction().1;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().0 + t * r.direction().0;
        let z = r.origin().2 + t * r.direction().2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(Hit::new(
            t,
            r.point_at_param(t),
            Vec3(0.0, 1.0, 0.0),
            &self.mat,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3(self.x0, self.k - EPS, self.z0),
            Vec3(self.x1, self.k + EPS, self.z1),
        ))
    }
}

pub struct YZRect<M: Scatterable> {
    y0: f32,
    z0: f32,
    y1: f32,
    z1: f32,

    // x = k => t = (k - a) / b
    k: f32,

    mat: M,
}

impl<M: Scatterable> YZRect<M> {
    pub fn new(y0: f32, z0: f32, y1: f32, z1: f32, k: f32, mat: M) -> YZRect<M> {
        YZRect {
            y0,
            z0,
            y1,
            z1,
            k,
            mat,
        }
    }
}

impl<M: Scatterable> Hittable for YZRect<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin().0) / r.direction().0;
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin().1 + t * r.direction().1;
        let z = r.origin().2 + t * r.direction().2;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(Hit::new(
            t,
            r.point_at_param(t),
            Vec3(1.0, 0.0, 0.0),
            &self.mat,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3(self.k - EPS, self.y0, self.z0),
            Vec3(self.k + EPS, self.y1, self.z1),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_rect_hit() {
        let rect = XYRect::new(-1.0, -1.0, 1.0, 1.0, 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), 0.0);
        let res = rect.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_some());
    }

    #[test]
    fn test_xz_rect_hit() {
        let rect = XZRect::new(-1.0, -1.0, 1.0, 1.0, 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), 0.0);
        let res = rect.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_some());
    }

    #[test]
    fn test_yz_rect_hit() {
        let rect = YZRect::new(-1.0, -1.0, 1.0, 1.0, 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), 0.0);
        let res = rect.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_some());
    }

    #[test]
    fn test_xy_rect_miss() {
        let rect = XYRect::new(-1.0, -1.0, 1.0, 1.0, 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(-1.0, -1.0, -1.0), 0.0);
        let res = rect.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_none());
    }

    #[test]
    fn test_xz_rect_miss() {
        let rect = XZRect::new(-1.0, -1.0, 1.0, 1.0, 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(-1.0, -1.0, -1.0), 0.0);
        let res = rect.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_none());
    }

    #[test]
    fn test_yz_rect_miss() {
        let rect = YZRect::new(-1.0, -1.0, 1.0, 1.0, 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(-1.0, -1.0, -1.0), 0.0);
        let res = rect.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_none());
    }

    struct TestMaterial {
        res: Option<(Ray, Vec3)>,
    }

    impl Scatterable for TestMaterial {
        fn scatter(&self, r: &Ray, hit: Hit) -> Option<(Ray, Vec3)> {
            None
        }
    }
}
