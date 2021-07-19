use image::{DynamicImage, GenericImageView};

use crate::color::Color;

use super::Checker;


#[derive(Debug, Clone, PartialEq)]
pub enum Texture {
    Color(Color),
    Checker(Checker),
    Image(DynamicImage)
}

impl Texture {
    pub fn color(&self, coord: &TextureCoord) -> Color {
        match self {
            Texture::Color(color) => color.clone(),
            Texture::Checker(checker) => checker.color(coord),
            Texture::Image(image) => {
                let (x, y) = coord.scale_wrap(coord.scale, image.width(), image.height());
                image.get_pixel(x, y).into()
            }
        }
    }
}

impl From<DynamicImage> for Texture {
    fn from(i: DynamicImage) -> Self {
        Texture::Image(i)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureCoord {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}

impl TextureCoord {
    pub fn new(x: f64, y: f64, scale: f64) -> Self {
        TextureCoord { x, y, scale }
    }

    pub fn scale_wrap(&self, scale: f64, max_x: u32, max_y: u32) -> (u32, u32) {
        (
            TextureCoord::wrap_v(self.x * scale, max_x),
            TextureCoord::wrap_v(self.y * scale, max_y),
        )
    }

    pub fn wrap(&self, max_x: u32, max_y: u32) -> (u32, u32) {
        self.scale_wrap(self.scale, max_x, max_y)
    }

    fn wrap_v(v: f64, bound: u32) -> u32 {
        let bound = bound as i32;

        let wrapped = ((v * bound as f64) as i32) % bound;

        if wrapped < 0 {
            (wrapped + bound) as u32
        } else {
            wrapped as u32
        }
    }
}
