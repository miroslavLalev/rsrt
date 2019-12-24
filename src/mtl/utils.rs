use crate::math::Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_nt: f32) -> Option<Vec3> {
    let v = v.as_unit();
    let dt = v.dot(n);
    let disc = 1.0 - ni_nt * ni_nt * (1.0 - dt * dt);
    if disc > 0.0 {
        Some(ni_nt * (v - n * dt) - n * disc.sqrt())
    } else {
        None
    }
}

pub fn schlick(rfn_ind: f32, cos: f32) -> f32 {
    let r0 = ((1.0 - rfn_ind) / (1.0 + rfn_ind)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
