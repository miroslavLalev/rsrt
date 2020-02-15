use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};

pub struct Sphere<M: Scatterable> {
    center: Vec3,
    r: f32,
    mat: M,
}

impl<M: Scatterable> Sphere<M> {
    pub fn new(center: Vec3, r: f32, mat: M) -> Sphere<M> {
        Sphere { center, r, mat }
    }

    pub fn get_uv(p: Vec3) -> (f32, f32) {
        let phi = p.2.atan2(p.0);
        let theta = p.1.asin();
        (
            1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI),
            (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI,
        )
    }
}

impl<M: Scatterable> Hittable for Sphere<M> {
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
            let (u, v) = Self::get_uv((p - self.center) / self.r);
            return Some(Hit::new(
                tmp,
                p,
                (p - self.center) / self.r,
                &self.mat,
                u,
                v,
            ));
        } else {
            let tmp = (-b + disc.sqrt()) / (2.0 * a);
            if tmp < t_max && tmp > t_min {
                let p = r.point_at_param(tmp);
                let (u, v) = Self::get_uv((p - self.center) / self.r);
                return Some(Hit::new(
                    tmp,
                    p,
                    (p - self.center) / self.r,
                    &self.mat,
                    u,
                    v,
                ));
            }
        }

        None
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3(self.r, self.r, self.r),
            self.center + Vec3(self.r, self.r, self.r),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_hit() {
        let s = Sphere::new(Vec3(1.0, 1.0, 1.0), 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0), 0.0);
        let res = s.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_some());
    }

    #[test]
    fn test_sphere_miss() {
        let s = Sphere::new(Vec3(1.0, 1.0, 1.0), 1.0, TestMaterial { res: None });
        let ray = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(-1.0, -1.0, -1.0), 0.0);
        let res = s.hit(&ray, 0.0, 1.0);
        assert_eq!(true, res.is_none());
    }

    #[test]
    fn test_sphere_bbox() {
        let s = Sphere::new(Vec3(1.0, 1.0, 1.0), 1.0, TestMaterial { res: None });
        let bbox = s
            .bounding_box(0.0, 0.0)
            .expect("bbox for sphere should always defined");
        assert_eq!(bbox.min(), Vec3(0.0, 0.0, 0.0));
        assert_eq!(bbox.max(), Vec3(2.0, 2.0, 2.0));
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
