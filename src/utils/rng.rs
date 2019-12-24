use rand::{distributions::uniform::SampleUniform, Rng};

use crate::math::Vec3;

pub fn uniform_in_range<T: SampleUniform>(low: T, high: T) -> T {
    rand::thread_rng().gen_range(low, high)
}

pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let v = 2.0 * Vec3(uniform_in_range(0.0, 1.0), uniform_in_range(0.0, 1.0), 0.0)
            - Vec3(1.0, 1.0, 0.0);
        if v.dot(v) >= 1.0 {
            break v;
        }
    }
}

pub fn rand_in_unit_sphere() -> Vec3 {
    loop {
        let v =
            2.0 * Vec3(
                uniform_in_range(0.0, 1.0),
                uniform_in_range(0.0, 1.0),
                uniform_in_range(0.0, 1.0),
            ) - Vec3(1.0, 1.0, 1.0);
        if v.sq_len() >= 1.0 {
            break v;
        }
    }
}
