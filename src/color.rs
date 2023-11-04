use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color {
    grey: u8,
}

impl Color {
    pub(crate) fn new_rand(rng: &mut impl Rng) -> Color {
        Color { grey: rng.gen_range(0..=255)}
    }

    pub(crate) fn distance(a: &Color, b: &Color) -> f32 {
        (a.grey as f32 - b.grey as f32).abs()
    }
}


impl From<Color> for raylib::prelude::Color {
    fn from(value: Color) -> Self {
        raylib::prelude::Color {
            r: 0,
            g: 0,
            b: value.grey,
            a: 255,
        }
    }
}