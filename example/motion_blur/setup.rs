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

    let objects: Vec<Box<dyn Hittable>> = vec![Box::new(MovSphere::new(
        Vec3(0.0, 0.0, 0.0),
        0.0,
        Vec3(0.0, 0.5, 0.0),
        1.0,
        1.0,
        Lambertian::new(ConstTexture::new(Vec3(0.5, 0.5, 0.0))),
    ))];

    (cam, HitVec::new(objects))
}

fn compute_background(r: &Ray) -> Vec3 {
    Vec3(0.7, 0.7, 0.7)
}
