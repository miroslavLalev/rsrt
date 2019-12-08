use super::super::math::Vec3;
use super::Ray;

pub struct Camera {
    origin: Vec3,
    low_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vec3(0.0, 0.0, 0.0),
            low_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.low_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
