use crate::math::Vec3;

/// Ray is a function P(t) = A + t * B that gives position along a line.
///     A - vector for ray origin
///     B - vector for ray direction
#[derive(Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    /// Returns a new ray for the given parameters.
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    /// Returns the origin of the ray.
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// Returns the direction of the ray.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    /// Returns ray function value vector for t - P(t) = A + t * B.
    pub fn point_at_param(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
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
