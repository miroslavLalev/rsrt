use crate::math::Vec3;

#[derive(Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: f32) -> Ray {
        Ray { a, b, time }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn point_at_param(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_pp() {
        let r = Ray::new(Vec3(1.0, 1.0, 1.0), Vec3(1.5, 1.5, 1.5), 0.0);

        assert_eq!(r.point_at_param(2.0), Vec3(4.0, 4.0, 4.0));
    }
}
