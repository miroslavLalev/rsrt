use crate::math::Vec3;
use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};

pub struct RotateY<H: Hittable> {
    hittable: H,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(hittable: H, angle: f32) -> RotateY<H> {
        let rad = (std::f32::consts::PI / 180.0) * angle;
        let sin_theta = rad.sin();
        let cos_theta = rad.cos();
        match hittable.bounding_box(0.0, 1.0) {
            None => RotateY {
                hittable,
                sin_theta,
                cos_theta,
                bbox: None,
            },
            Some(bbox) => RotateY::rotate_bbox(hittable, cos_theta, sin_theta, bbox)
        }
    }

    fn rotate_bbox(hittable: H, cos_theta: f32, sin_theta: f32, bbox: AABB) -> RotateY<H> {
        let mut min = Vec3(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vec3(std::f32::MIN, std::f32::MIN, std::f32::MIN);

        for i in 0..2 {
            let i = i as f32;
            for j in 0..2 {
                let j = j as f32;
                for k in 0..2 {
                    let k = k as f32;

                    let x = i * bbox.max().0 + (1.0 - i) * bbox.min().0;
                    let y = j * bbox.max().1 + (1.0 - j) * bbox.min().1;
                    let z = k * bbox.max().2 + (1.0 - k) * bbox.min().2;
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tmp = Vec3(new_x, y, new_z);
                    min.0 = tmp.0.min(min.0);
                    min.1 = tmp.1.min(min.1);
                    min.2 = tmp.2.min(min.2);

                    max.0 = tmp.0.max(max.0);
                    max.1 = tmp.1.max(max.1);
                    max.2 = tmp.2.max(max.2);
                }
            }
        }

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bbox: Some(AABB::new(min, max)),
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut origin = r.origin();
        origin.0 = self.cos_theta * r.origin().0 - self.sin_theta * r.origin().2;
        origin.2 = self.sin_theta * r.origin().0 + self.cos_theta * r.origin().2;

        let mut direction = r.direction();
        direction.0 = self.cos_theta * r.direction().0 - self.sin_theta * r.direction().2;
        direction.2 = self.sin_theta * r.direction().0 + self.cos_theta * r.direction().2;

        let rotated_ray = Ray::new(origin, direction, r.time());
        if let Some(hit) = self.hittable.hit(&rotated_ray, t_min, t_max) {
            let mut p = hit.p();
            p.0 = self.cos_theta * hit.p().0 + self.sin_theta * hit.p().2;
            p.2 = -self.sin_theta * hit.p().0 + self.cos_theta * hit.p().2;

            let mut n = hit.n();
            n.0 = self.cos_theta * hit.n().0 + self.sin_theta * hit.n().2;
            n.2 = -self.sin_theta * hit.n().0 + self.cos_theta * hit.n().2;

            return Some(Hit::new(hit.t(), p, n, hit.mat_ref(), hit.u(), hit.v()));
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}
