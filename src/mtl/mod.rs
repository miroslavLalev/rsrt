pub use dielectric::Dielectric;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use light_source::LightDiffuse;
pub use metal::Metal;
pub use scatterable::Scatterable;

mod dielectric;
mod isotropic;
mod lambertian;
mod light_source;
mod metal;
mod scatterable;
mod utils;
