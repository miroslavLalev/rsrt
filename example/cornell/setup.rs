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

    let red = Lambertian::new(ConstTexture::new(Vec3(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstTexture::new(Vec3(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstTexture::new(Vec3(0.12, 0.45, 0.15)));
    let light = LightDiffuse::new(ConstTexture::new(Vec3(15.0, 15.0, 15.0)));

    let mut objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(FlipNormals::new(YZRect::new(
            0.0, 0.0, 555.0, 555.0, 555.0, green,
        ))),
        Box::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)),
        Box::new(XZRect::new(213.0, 227.0, 343.0, 332.0, 554.0, light)),
        Box::new(FlipNormals::new(XZRect::new(
            0.0,
            0.0,
            555.0,
            555.0,
            555.0,
            white.clone(),
        ))),
        Box::new(XZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, white.clone())),
        Box::new(FlipNormals::new(XYRect::new(
            0.0,
            0.0,
            555.0,
            555.0,
            555.0,
            white.clone(),
        ))),
    ];

    let b1 = Translate::new(
        RotateY::new(
            RectBox::new(
                Vec3(0.0, 0.0, 0.0),
                Vec3(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3(130.0, 0.0, 65.0),
    );
    let b2 = Translate::new(
        RotateY::new(
            RectBox::new(
                Vec3(0.0, 0.0, 0.0),
                Vec3(165.0, 330.0, 165.0),
                white.clone(),
            ),
            15.0,
        ),
        Vec3(265.0, 0.0, 295.0),
    );

    objects.push(Box::new(b1));
    objects.push(Box::new(b2));

    (cam, HitVec::new(objects))
}

fn compute_background(r: &Ray) -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
}