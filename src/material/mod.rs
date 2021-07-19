pub use crate::material::checker::Checker;
pub use crate::material::texture::{Texture, TextureCoord};

use crate::color::Color;

mod checker;
mod texture;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Surface {
    Diffuse,
    Reflective(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub texture: Texture,
    pub albedo: f64,
    pub surface: Surface,
    pub scale: f64,
}

impl Material {
    pub fn new(texture: impl Into<Texture>) -> Self {
        Material {
            texture: texture.into(),
            albedo: 1.0,
            surface: Surface::Diffuse,
            scale: 1.0,
        }
    }

    pub fn albedo(mut self, albedo: f64) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn surface(mut self, surface: Surface) -> Self {
        self.surface = surface;
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn color(&self, coord: &TextureCoord) -> Color {
        self.texture.color(coord)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Color::new(0.0, 0.9, 0.2))
    }
}
