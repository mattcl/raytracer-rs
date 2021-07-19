use crate::color::Color;

use super::{Texture, TextureCoord};

#[derive(Debug, Clone, PartialEq)]
pub struct Checker {
    prime: Color,
    secondary: Color,
}

impl Checker {
    pub fn new(color: Color) -> Self {
        Checker {
            prime: color,
            secondary: color * 0.8,
        }
    }

    pub fn secondary(mut self, color: Color) -> Self {
        self.secondary = color;
        self
    }

    pub fn color(&self, coord: &TextureCoord) -> Color {
        let mut x = coord.x * coord.scale % 1.0;
        if x < 0.0 {
            x += 1.0;
        }

        let mut y = coord.y * coord.scale % 1.0;
        if y < 0.0 {
            y += 1.0;
        }

        if (x > 0.5) ^ (y > 0.5) {
            self.prime.clone()
        } else {
            self.secondary.clone()
        }
    }
}

impl Default for Checker {
    fn default() -> Self {
        Checker::new(Color::new(0.0, 1.0, 0.0))
    }
}

impl From<Checker> for Texture {
    fn from(c: Checker) -> Self {
        Texture::Checker(c)
    }
}
