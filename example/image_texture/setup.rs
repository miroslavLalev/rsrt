fn prepare_scene(nx: u32, ny: u32) -> (Camera, HitVec) {
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let img_bytes = include_bytes!("earthmap.jpg");
    let img = image::load_from_memory(img_bytes.as_ref()).expect("failed to load image");

    let spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            Vec3(0.0, 0.0, 0.0),
            2.0,
            Lambertian::new(ImageTexture::new(img)),
        )),
        Box::new(XYRect::new(
            -2.0,
            -2.0,
            2.0,
            2.0,
            -4.0,
            LightDiffuse::new(CheckerTexture::new(
                ConstTexture::new(Vec3(0.0, 1.0, 1.0)),
                ConstTexture::new(Vec3(1.0, 1.0, 0.0)),
            )),
        )),
        Box::new(Sphere::new(
            Vec3(6.0, 3.0, 3.0),
            0.5,
            LightDiffuse::new(ConstTexture::new(Vec3(1.0, 1.0, 0.0))),
        )),
    ];

    (cam, HitVec::new(spheres))
}

fn compute_background(r: &Ray) -> Vec3 {
    let unit = r.direction().as_unit();
    let t = 0.5 * (unit.1 + 1.0);
    (1.0 - t) * Vec3(0.0, 0.0, 0.0) + t * Vec3(0.7, 0.7, 0.7)
}