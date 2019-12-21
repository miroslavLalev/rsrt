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

pub struct Dielectric {
    rfn_ind: f32,
}

impl Dielectric {
    pub fn new(rfn_ind: f32) -> Dielectric {
        Dielectric { rfn_ind }
    }

    fn refract(v: Vec3, n: Vec3, ni_nt: f32) -> Option<Vec3> {
        let v = v.as_unit();
        let dt = v.dot(n);
        let disc = 1.0 - ni_nt * ni_nt * (1.0 - dt * dt);
        if disc > 0.0 {
            Some(ni_nt * (v - n * dt) - n * disc.sqrt())
        } else {
            None
        }
    }

    fn schlick(&self, cos: f32) -> f32 {
        let r0 = ((1.0 - self.rfn_ind) / (1.0 + self.rfn_ind)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    fn rand_float() -> f32 {
        rand::thread_rng().gen_range(0.0, 1.0)
    }
}

impl Scatterable<Dielectric> for Dielectric {
    fn scatter(&self, r: &Ray, hit: Hit<Dielectric>) -> Option<(Ray, Vec3)> {
        let reflected = Metal::reflect(r.direction(), hit.n());

        let (out_norm, ni_nt, cos) = if r.direction().dot(hit.n()) > 0.0 {
            (
                -hit.n(),
                self.rfn_ind,
                self.rfn_ind * r.direction().dot(hit.n()) / r.direction().len(),
            )
        } else {
            (
                hit.n(),
                1.0 / self.rfn_ind,
                -r.direction().dot(hit.n()) / r.direction().len(),
            )
        };

        let (refracted, reflect_prob) = match Dielectric::refract(r.direction(), out_norm, ni_nt) {
            Some(refracted) => (Some(refracted), self.schlick(cos)),
            None => (None, 1.0),
        };

        if Dielectric::rand_float() < reflect_prob {
            Some((Ray::new(hit.p(), refracted?), Vec3(1.0, 1.0, 1.0)))
        } else {
            Some((Ray::new(hit.p(), reflected), Vec3(1.0, 1.0, 1.0)))
        }
    }
}
