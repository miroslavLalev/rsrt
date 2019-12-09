use rand::Rng;

use super::super::math::Vec3;
use super::Ray;
use crate::trace::Hit;

pub trait Scatterable<M: Scatterable<M>> {
    fn scatter(&self, r: &Ray, hit: Hit<M>) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    color: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { color }
    }

    fn rand_float() -> f32 {
        rand::thread_rng().gen_range(0.0, 1.0)
    }

    fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let v =
                2.0 * Vec3(
                    Lambertian::rand_float(),
                    Lambertian::rand_float(),
                    Lambertian::rand_float(),
                ) - Vec3(1.0, 1.0, 1.0);
            if v.sq_len() >= 1.0 {
                break v;
            }
        }
    }
}

impl Scatterable<Lambertian> for Lambertian {
    fn scatter(&self, _: &Ray, hit: Hit<Lambertian>) -> Option<(Ray, Vec3)> {
        let target = hit.p() + hit.n() + Lambertian::rand_in_unit_sphere();
        Some((Ray::new(hit.p(), target - hit.p()), self.color))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: f32::min(fuzz, 1.0),
        }
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let v =
                2.0 * Vec3(
                    Lambertian::rand_float(),
                    Lambertian::rand_float(),
                    Lambertian::rand_float(),
                ) - Vec3(1.0, 1.0, 1.0);
            if v.sq_len() >= 1.0 {
                break v;
            }
        }
    }
}

impl Scatterable<Metal> for Metal {
    fn scatter(&self, r: &Ray, hit: Hit<Metal>) -> Option<(Ray, Vec3)> {
        let reflected = Metal::reflect(r.direction().as_unit(), hit.n());
        let scattered = Ray::new(
            hit.p(),
            reflected + self.fuzz * Metal::rand_in_unit_sphere(),
        );
        if scattered.direction().dot(hit.n()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
