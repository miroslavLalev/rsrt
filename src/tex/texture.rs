use crate::math::Vec3;
use image::{DynamicImage, GenericImageView, ImageResult};

pub trait Wrappable {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

pub struct ConstTexture {
    color: Vec3,
}

impl ConstTexture {
    pub fn new(color: Vec3) -> ConstTexture {
        ConstTexture { color }
    }
}

impl Wrappable for ConstTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    t0: Box<dyn Wrappable>,
    t1: Box<dyn Wrappable>,
}

impl CheckerTexture {
    pub fn new(t0: Box<dyn Wrappable>, t1: Box<dyn Wrappable>) -> CheckerTexture {
        CheckerTexture { t0, t1 }
    }
}

impl Wrappable for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.0).sin() * (10.0 * p.1).sin() * (10.0 * p.2).sin();
        if sines < 0.0 {
            self.t0.value(u, v, p)
        } else {
            self.t1.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(image: DynamicImage) -> ImageTexture {
        ImageTexture { image }
    }
}

impl Wrappable for ImageTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let (nx, ny) = self.image.dimensions();
        let i = ((u * nx as f32) as u32).max(0).min(nx - 1);
        let j = (((1.0 - v) * ny as f32) as u32).max(0).min(ny - 1);
        let color = self.image.get_pixel(i as u32, j as u32).0;
        Vec3(
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
        )
    }
}
