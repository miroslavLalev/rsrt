#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    #[inline]
    pub fn len(&self) -> f32 {
        self.sq_len().sqrt()
    }

    #[inline]
    pub fn sq_len(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[inline]
    pub fn as_unit(self) -> Vec3 {
        self / self.len()
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f32) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f32) -> Vec3 {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_operations() {
        let v1 = Vec3(1.0, 1.0, 1.0);
        let v2 = Vec3(2.0, 2.0, 2.0);

        assert_eq!(v1.sq_len(), 3.0);
        assert_eq!(v1.len(), 3.0f32.sqrt());
        assert_eq!(v2.sq_len(), 12.0);
        assert_eq!(v2.len(), 12.0f32.sqrt());

        assert_eq!(v1 * 2.0, Vec3(2.0, 2.0, 2.0));
        assert_eq!(2.0 * v1, Vec3(2.0, 2.0, 2.0));
        assert_eq!(v2 / 2.0, Vec3(1.0, 1.0, 1.0));
        assert_eq!(-v2, Vec3(-2.0, -2.0, -2.0));

        assert_eq!(v1 + v2, Vec3(3.0, 3.0, 3.0));
        assert_eq!(v1 - v2, Vec3(-1.0, -1.0, -1.0));
        assert_eq!(v1 * v2, Vec3(2.0, 2.0, 2.0));
        assert_eq!(v2 / v1, Vec3(2.0, 2.0, 2.0));

        assert_eq!(v1.cross(v2), Vec3(0.0, 0.0, 0.0));
        assert_eq!(v1.dot(v2), 6.0);
    }
}
