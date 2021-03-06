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
    let nthreshold = 0.0001;
    let strategy = Bucket::new(nx, ny, 4);

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
                let u = (x as f32 + uniform_in_range(0.0, 1.0)) / nx as f32;
                let v = ((ny - y) as f32 + uniform_in_range(0.0, 1.0)) / ny as f32;
                let mut col = Vec3(0.0, 0.0, 0.0);
                let mut i = 0;
                let iters = loop {
                    let newcol = col + color(cam.get_ray(u, v), &hit_vec, 0);
                    if (newcol.0 - col.0).abs() < nthreshold && (newcol.1 - col.1).abs() < nthreshold && (newcol.2 - col.2) < nthreshold {
                        // noise threshold reached
                        break i;
                    }
                    col = newcol;
                    if i == ns-1 {
                        break i;
                    }
                    i+=1;
                };
                col = col / (iters+1) as f32;
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
        "c://Users/User/Desktop/watery_-1.jpeg",
        image::ImageFormat::JPEG,
    )
}

fn prepare_scene(nx: u32, ny: u32) -> (Camera, HitVec) {
    let lookfrom = Vec3(50.0, 0.0, 0.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let focus_dist = 4.0;
    let aperture = 0.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        4.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3(-6.0, 0.0, 0.0), 1.0, Lambertian::new(ConstTexture::new(Vec3(0.8, 0.1, 0.1))))),
        Box::new(Sphere::new(Vec3(-5.0, -1.0, 0.0), 1.0, Dielectric::new(-1.33, ConstTexture::new(Vec3(0.1, 0.4, 0.4))))),
    ];

    (cam, HitVec::new(objects))
}

fn compute_background(_: &Ray) -> Vec3 {
    Vec3(0.7, 0.7, 0.7)
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
