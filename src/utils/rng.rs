use rand::{distributions::uniform::SampleUniform, Rng};

use crate::math::Vec3;

/// Returns uniformly distributed random number from the interval [low, high).
pub fn uniform_in_range<T: SampleUniform>(low: T, high: T) -> T {
    rand::thread_rng().gen_range(low, high)
}

/// Returns random vector within a disk.
pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let v = Vec3(uniform_in_range(0.0, 1.0), uniform_in_range(0.0, 1.0), 0.0);
        if v.sq_len() < 1.0 {
            break v;
        }
    }
}

/// Returns random vector within a sphere.
pub fn rand_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3(
            uniform_in_range(0.0, 1.0),
            uniform_in_range(0.0, 1.0),
            uniform_in_range(0.0, 1.0),
        );
        if v.sq_len() < 1.0 {
            break v;
        }
    }
}
