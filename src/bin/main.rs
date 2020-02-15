extern crate rsrt;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use threadpool::ThreadPool;

use rsrt::math::Vec3;
use rsrt::mtl::{Dielectric, Lambertian, LightDiffuse, Metal};
use rsrt::obj::transform::{FlipNormals, RotateY, Translate};
use rsrt::obj::{ConstDensity, MovSphere, RectBox, Sphere, XYRect, XZRect, YZRect};
use rsrt::strategy::Bucket;
use rsrt::tex::{CheckerTexture, ConstTexture, ImageTexture};
use rsrt::trace::{BVHNode, Camera, HitVec, Hittable, Ray};
use rsrt::utils::rng::uniform_in_range;
use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    let nx = 1000;
    let ny = 1000;
    let ns = 100;
    let strategy = Bucket::new(nx, ny, 10);

    let (cam, hit_vec) = prepare_scene(nx, ny);

    let cam = Arc::new(cam);
    let hit_vec = Arc::new(hit_vec);
    let pool = ThreadPool::new(3);

    let pixels_data = Arc::new(Mutex::new(HashMap::new()));

    let mut buf = image::ImageBuffer::new(nx, ny);

    for items in strategy {
        let cam = Arc::clone(&cam);
        let hit_vec = Arc::clone(&hit_vec);
        let pixels_data = pixels_data.clone();

        pool.execute(move || {
            for (y, x) in items {
                let mut col = Vec3(0.0, 0.0, 0.0);
                for _ in 0..ns {
                    let u = (x as f32 + uniform_in_range(0.0, 1.0)) / nx as f32;
                    let v = ((ny - y) as f32 + uniform_in_range(0.0, 1.0)) / ny as f32;

                    let r = cam.get_ray(u, v);
                    col = col + color(r, &hit_vec, 0);
                }
                col = col / ns as f32;
                col = Vec3(
                    col.0.sqrt().min(1.0),
                    col.1.sqrt().min(1.0),
                    col.2.sqrt().min(1.0),
                );

                let mut pixels = pixels_data.lock().unwrap();
                pixels.insert((x, y), col);
            }
        });
    }

    pool.join();

    let pixels_data = pixels_data.lock().unwrap();
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let col = pixels_data
            .get(&(x, y))
            .unwrap_or_else(|| panic!("missing {}, {} in params", x, y));

        let r = (255.99 * col.0) as u8;
        let g = (255.99 * col.1) as u8;
        let b = (255.99 * col.2) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    buf.save_with_format(
        "c://Users/User/Desktop/final.jpeg",
        image::ImageFormat::JPEG,
    )
}

fn prepare_scene(nx: u32, ny: u32) -> (Camera, HitVec) {
    let lookfrom = Vec3(278.0, 278.0, -800.0);
    let lookat = Vec3(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    let mut box1: Vec<Rc<dyn Hittable>> = Vec::new();
    let white = Lambertian::new(ConstTexture::new(Vec3(0.73, 0.73, 0.73)));
    let ground = Lambertian::new(ConstTexture::new(Vec3(0.48, 0.83, 0.53)));

    for i in 0..20 {
        let i = i as f32;
        for j in 0..20 {
            let j = j as f32;

            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (uniform_in_range(0.0f32, 1.0f32) + 0.001);
            let z1 = z0 + w;
            box1.push(Rc::new(RectBox::new(
                Vec3(x0, y0, z0),
                Vec3(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    objects.push(Box::new(BVHNode::new(box1, 0.0, 1.0)));

    let light = LightDiffuse::new(ConstTexture::new(Vec3(7.0, 7.0, 7.0)));
    objects.push(Box::new(XZRect::new(
        123.0, 147.0, 423.0, 412.0, 554.0, light,
    )));

    let center = Vec3(400.0, 400.0, 200.0);
    objects.push(Box::new(MovSphere::new(
        center,
        0.0,
        center + Vec3(30.0, 0.0, 0.0),
        1.0,
        50.0,
        Lambertian::new(ConstTexture::new(Vec3(0.7, 0.3, 0.1))),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Vec3(0.8, 0.8, 0.9), 10.0),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5),
    )));
    objects.push(Box::new(ConstDensity::new(
        Sphere::new(Vec3(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5)),
        0.2,
        Lambertian::new(ConstTexture::new(Vec3(0.2, 0.4, 0.9))),
    )));
    objects.push(Box::new(ConstDensity::new(
        Sphere::new(Vec3(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5)),
        0.0001,
        Lambertian::new(ConstTexture::new(Vec3(1.0, 1.0, 1.0))),
    )));

    let mut box2: Vec<Rc<dyn Hittable>> = Vec::new();
    for j in 0..1000 {
        box2.push(Rc::new(Sphere::new(
            Vec3(
                165.0f32 * uniform_in_range(0.0, 1.0),
                165.0f32 * uniform_in_range(0.0, 1.0),
                165.0f32 * uniform_in_range(0.0, 1.0),
            ),
            10.0,
            white.clone(),
        )));
    }
    objects.push(Box::new(Translate::new(
        RotateY::new(BVHNode::new(box2, 0.0, 1.0), 15.0),
        Vec3(-100.0, 270.0, 395.0),
    )));

    (cam, HitVec::new(objects))
}

fn compute_background(r: &Ray) -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
}

fn color(r: Ray, hit_vec: &HitVec, depth: u8) -> Vec3 {
    if let Some(hit) = hit_vec.hit(&r, 0.001, std::f32::MAX) {
        let emitted = hit.mat_ref().emitted(hit.u(), hit.v(), hit.p());
        if depth > 50 {
            return emitted;
        }

        if let Some((r, col)) = hit.scatter(&r) {
            return emitted + col * color(r, hit_vec, depth + 1);
        } else {
            return emitted;
        }
    }

    compute_background(&r)
}
