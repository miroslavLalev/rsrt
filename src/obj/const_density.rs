use crate::math::Vec3;
use crate::mtl::Scatterable;
use crate::obj::AABB;
use crate::trace::{Hit, Hittable, Ray};
use crate::utils::rng::uniform_in_range;

pub struct ConstDensity<H: Hittable, S: Scatterable> {
    hittable: H,
    density: f32,
    phase_fn: S,
}

impl<H: Hittable, S: Scatterable> ConstDensity<H, S> {
    pub fn new(hittable: H, density: f32, phase_fn: S) -> ConstDensity<H, S> {
        ConstDensity {
            hittable,
            density,
            phase_fn,
        }
    }
}

impl<H: Hittable, S: Scatterable> Hittable for ConstDensity<H, S> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if let Some(hit1) = self.hittable.hit(r, std::f32::MIN, std::f32::MAX) {
            if let Some(hit2) = self.hittable.hit(r, hit1.t() + 0.0001, std::f32::MAX) {
                let hit1 = Hit::new(
                    hit1.t().max(t_min),
                    hit1.p(),
                    hit1.n(),
                    hit1.mat_ref(),
                    hit1.u(),
                    hit1.v(),
                );
                let hit2 = Hit::new(
                    hit2.t().min(t_max),
                    hit2.p(),
                    hit2.n(),
                    hit2.mat_ref(),
                    hit2.u(),
                    hit2.v(),
                );

                if hit1.t() >= hit2.t() {
                    return None;
                }

                let distance_in_boundary = (hit2.t() - hit1.t()) * r.direction().len();
                let hit_distance = -(1.0 / self.density) * uniform_in_range(0.0f32, 1.0f32).ln();
                if hit_distance < distance_in_boundary {
                    let t = hit1.t() + hit_distance / r.direction().len();
                    return Some(Hit::new(
                        t,
                        r.point_at_param(t),
                        Vec3(1.0, 0.0, 0.0),
                        &self.phase_fn,
                        hit2.u(),
                        hit2.v(),
                    ));
                }
            }
        }

        None
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABB> {
        self.hittable.bounding_box(t_min, t_max)
    }
}
