extern crate rsrt;

use rsrt::math::Vec3;
use rsrt::mtl::{Dielectric, Lambertian, Metal};
use rsrt::trace::{Camera, Hittable, Ray, Sphere};
use rsrt::utils::rng::uniform_in_range;

fn main() -> Result<(), std::io::Error> {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let cam = Camera::new();

    let mut buf = image::ImageBuffer::new(nx, ny);
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let mut col = Vec3(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f32 + uniform_in_range(0.0, 1.0)) / nx as f32;
            let v = ((ny - y) as f32 + uniform_in_range(0.0, 1.0)) / ny as f32;

            let r = cam.get_ray(u, v);
            col = col + color(r, 0);
        }
        col = col / ns as f32;
        col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());

        let r = (255.99 * col.0) as u8;
        let g = (255.99 * col.1) as u8;
        let b = (255.99 * col.2) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    //    buf.save_with_format(
    //        "c://Users/User/Desktop/image2_fix.jpeg",
    //        image::ImageFormat::JPEG,
    //    )
    buf.save_with_format("/Users/miro/Desktop/image", image::ImageFormat::JPEG)
}

fn color(r: Ray, depth: u8) -> Vec3 {
    let s = Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        //        Lambertian::new(Vec3(0.0, 0.4, 0.0)),
        Dielectric::new(1.5),
    );
    if let Some(hit) = s.hit(&r, 0.001, std::f32::MAX) {
        if depth > 50 {
            return Vec3(0.0, 0.0, 0.0);
        }
        if let Some((r, col)) = hit.scatter(&r) {
            return col * color(r, depth + 1);
        }
    }

    let s2 = Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Vec3(0.0, 0.6, 0.0)),
        //        Metal::new(Vec3(0.6, 0.0, 0.0), 0.0),
        //        Dielectric::new(0.3),
    );
    if let Some(hit) = s2.hit(&r, 0.001, std::f32::MAX) {
        if depth > 50 {
            return Vec3(0.0, 0.0, 0.0);
        }
        if let Some((r, col)) = hit.scatter(&r) {
            return col * color(r, depth + 1);
        }
    }

    let unit = r.direction().as_unit();
    let t = 0.5 * (unit.1 + 1.0);
    (1.0 - t) * Vec3(0.0, 0.0, 0.0) + t * Vec3(0.7, 0.7, 0.7)
}
