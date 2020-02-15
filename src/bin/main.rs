extern crate rsrt;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use threadpool::ThreadPool;

use rsrt::math::Vec3;
use rsrt::mtl::{Dielectric, Lambertian, LightDiffuse, Metal};
use rsrt::obj::transform::{FlipNormals, RotateY, Translate};
use rsrt::obj::{MovSphere, RectBox, Sphere, XYRect, XZRect, YZRect};
use rsrt::strategy::Bucket;
use rsrt::tex::{CheckerTexture, ConstTexture, ImageTexture};
use rsrt::trace::{Camera, HitVec, Hittable, Ray};
use rsrt::utils::rng::uniform_in_range;

fn main() -> Result<(), std::io::Error> {
    let nx = 555;
    let ny = 555;
    let ns = 1000;
    let strategy = Bucket::new(nx, ny, 24);

    //    let lookfrom = Vec3(20.0, 2.0, 3.0);
    //    let lookat = Vec3(0.0, 0.0, 0.0);
    //    let focus_dist = 10.0;
    //    let aperture = 0.0;
    //    let cam = Arc::new(Camera::new(
    //        lookfrom,
    //        lookat,
    //        Vec3(0.0, 1.0, 0.0),
    //        20.0,
    //        nx as f32 / ny as f32,
    //        aperture,
    //        focus_dist,
    //        0.0,
    //        1.0,
    //    ));

    let lookfrom = Vec3(278.0, 278.0, -800.0);
    let lookat = Vec3(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let cam = Arc::new(Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    ));

    //    let mut spheres: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(
    //        Vec3(0.0, -1000.0, 0.0),
    //        1000.0,
    //        Lambertian::new(Box::new(CheckerTexture::new(
    //            Box::new(ConstTexture::new(Vec3(0.2, 0.3, 0.1))),
    //            Box::new(ConstTexture::new(Vec3(0.9, 0.9, 0.9))),
    //        ))),
    //    ))];
    //
    //    for a in -10..10 {
    //        for b in -10..10 {
    //            let choose_mat = uniform_in_range(0.0, 1.0);
    //            let center = Vec3(
    //                a as f32 + 0.9 * uniform_in_range(0.0, 1.0),
    //                0.2,
    //                b as f32 + 0.9 * uniform_in_range(0.0, 1.0),
    //            );
    //
    //            if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
    //                if choose_mat < 0.8 {
    //                    spheres.push(Box::new(MovSphere::new(
    //                        center,
    //                        0.0,
    //                        center + Vec3(0.0, 0.5 * uniform_in_range(0.0, 1.0), 0.0),
    //                        1.0,
    //                        0.2,
    //                        Lambertian::new(Box::new(ConstTexture::new(Vec3(
    //                            uniform_in_range(0.0, 1.0) * uniform_in_range(0.0, 1.0),
    //                            uniform_in_range(0.0, 1.0) * uniform_in_range(0.0, 1.0),
    //                            uniform_in_range(0.0, 1.0) * uniform_in_range(0.0, 1.0),
    //                        )))),
    //                    )))
    //                } else if choose_mat < 0.95 {
    //                    spheres.push(Box::new(Sphere::new(
    //                        center,
    //                        0.2,
    //                        Metal::new(
    //                            Vec3(
    //                                0.5 * (1.0 + uniform_in_range(0.0, 1.0)),
    //                                0.5 * (1.0 + uniform_in_range(0.0, 1.0)),
    //                                0.5 * (1.0 + uniform_in_range(0.0, 1.0)),
    //                            ),
    //                            0.5 * uniform_in_range(0.0, 1.0),
    //                        ),
    //                    )))
    //                } else {
    //                    spheres.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))))
    //                }
    //            }
    //        }
    //    }

    //    let img_bytes = include_bytes!("earthmap.jpg");
    //    let img = image::load_from_memory(img_bytes.as_ref()).expect("failed to load image");
    //
    //    let spheres: Vec<Box<dyn Hittable>> = vec![
    //        Box::new(Sphere::new(
    //            Vec3(0.0, 0.0, 0.0),
    //            2.0,
    //            Lambertian::new(Box::new(ImageTexture::new(img))),
    //        )),
    //        Box::new(XYRect::new(
    //            -2.0,
    //            -2.0,
    //            2.0,
    //            2.0,
    //            -4.0,
    //            LightDiffuse::new(Box::new(CheckerTexture::new(
    //                Box::new(ConstTexture::new(Vec3(0.0, 1.0, 1.0))),
    //                Box::new(ConstTexture::new(Vec3(1.0, 1.0, 0.0))),
    //            ))),
    //        )),
    //        Box::new(Sphere::new(
    //            Vec3(6.0, 3.0, 3.0),
    //            0.5,
    //            LightDiffuse::new(Box::new(ConstTexture::new(Vec3(1.0, 1.0, 0.0)))),
    //        )),
    //    ];

    let red = Lambertian::new(ConstTexture::new(Vec3(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstTexture::new(Vec3(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstTexture::new(Vec3(0.12, 0.45, 0.15)));
    let light = LightDiffuse::new(Box::new(ConstTexture::new(Vec3(15.0, 15.0, 15.0))));

    let spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(FlipNormals::new(Box::new(YZRect::new(
            0.0, 0.0, 555.0, 555.0, 555.0, green,
        )))),
        Box::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)),
        Box::new(XZRect::new(213.0, 227.0, 343.0, 332.0, 554.0, light)),
        Box::new(FlipNormals::new(Box::new(XZRect::new(
            0.0,
            0.0,
            555.0,
            555.0,
            555.0,
            white.clone(),
        )))),
        Box::new(XZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, white.clone())),
        Box::new(FlipNormals::new(Box::new(XYRect::new(
            0.0,
            0.0,
            555.0,
            555.0,
            555.0,
            white.clone(),
        )))),
        //        Box::new(RectBox::new(
        //            Vec3(130.0, 0.0, 65.0),
        //            Vec3(295.0, 165.0, 230.0),
        //            white.clone(),
        //        )),
        //        Box::new(RectBox::new(
        //            Vec3(265.0, 0.0, 295.0),
        //            Vec3(430.0, 330.0, 460.0),
        //            white.clone(),
        //        )),
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(RectBox::new(
                    Vec3(0.0, 0.0, 0.0),
                    Vec3(165.0, 165.0, 165.0),
                    white.clone(),
                )),
                -18.0,
            )),
            Vec3(130.0, 0.0, 65.0),
        )),
        Box::new(Translate::new(
            Box::new(RotateY::new(
                Box::new(RectBox::new(
                    Vec3(0.0, 0.0, 0.0),
                    Vec3(165.0, 330.0, 165.0),
                    white.clone(),
                )),
                15.0,
            )),
            Vec3(265.0, 0.0, 295.0),
        )),
    ];

    let hit_vec = Arc::new(HitVec::new(spheres));
    let pool = ThreadPool::new(4);

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
                    col = col + color(r, hit_vec.as_ref(), 0);
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
            .expect(format!("missing {}, {} in params", x, y).as_str());

        let r = (255.99 * col.0) as u8;
        let g = (255.99 * col.1) as u8;
        let b = (255.99 * col.2) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    buf.save_with_format(
        "c://Users/User/Desktop/cornell_full.jpeg",
        image::ImageFormat::JPEG,
    )
    // buf.save_with_format("/Users/miro/Desktop/image", image::ImageFormat::JPEG)
}

//fn color(r: Ray, hit_vec: &HitVec, depth: u8) -> Vec3 {
//    if let Some(hit) = hit_vec.hit(&r, 0.001, std::f32::MAX) {
//        if depth > 50 {
//            return Vec3(0.0, 0.0, 0.0);
//        }
//        if let Some((r, col)) = hit.scatter(&r) {
//            return col * color(r, hit_vec, depth + 1);
//        }
//    }
//
//    let unit = r.direction().as_unit();
//    let t = 0.5 * (unit.1 + 1.0);
//    (1.0 - t) * Vec3(0.0, 0.0, 0.0) + t * Vec3(0.7, 0.7, 0.7)
//}

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

    Vec3(0.0, 0.0, 0.0)
}
