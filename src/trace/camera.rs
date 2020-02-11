use crate::math::Vec3;
use crate::trace::Ray;

use crate::utils::rng::{rand_in_unit_disk, uniform_in_range};

pub struct Camera {
    origin: Vec3,
    low_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    time_begin: f32,
    time_end: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time_begin: f32,
        time_end: f32,
    ) -> Camera {
        let thetha = fov * std::f32::consts::PI / 180.0;
        let half_height = (thetha / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).as_unit();
        let u = vup.cross(w).as_unit();
        let v = w.cross(u);
        Camera {
            origin: lookfrom,
            low_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time_begin,
            time_end,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * rand_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray::new(
            self.origin + offset,
            self.low_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            self.time_begin + uniform_in_range(0.0, 1.0) * (self.time_end - self.time_begin),
        )
    }
}

// &Camera can be shared between threads.
unsafe impl Sync for Camera {}
