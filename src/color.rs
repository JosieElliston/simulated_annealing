use rand::prelude::*;

// #[derive(Debug, Clone, Copy)]
// pub(crate) struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// impl Color {
//     pub(crate) fn new_rand(rng: &mut impl Rng) -> Color {
//         Color {
//             r: rng.gen_range(0..=255),
//             g: rng.gen_range(0..=255),
//             b: rng.gen_range(0..=255),
//         }
//     }

//     pub(crate) fn distance(a: &Color, b: &Color) -> f32 {
//         ((a.r as f32 - b.r as f32).powi(2)
//             + (a.g as f32 - b.g as f32).powi(2)
//             + (a.b as f32 - b.b as f32).powi(2))
//         .sqrt()

//         // (a.r as f32 - b.r as f32)
//         //     .abs()
//         //     .max((a.g as f32 - b.g as f32).abs())
//         //     .max((a.b as f32 - b.b as f32).abs())

//         // (a.r as f32 - b.r as f32)
//         //     .abs()
//         //     .min((a.g as f32 - b.g as f32).abs())
//         //     .min((a.b as f32 - b.b as f32).abs())

//         // (a.r as f32 - b.r as f32).abs() + (a.g as f32 - b.g as f32).abs() + (a.b as f32 - b.b as f32).abs()
//     }
// }

// impl From<Color> for raylib::prelude::Color {
//     fn from(value: Color) -> Self {
//         raylib::prelude::Color {
//             r: value.r,
//             g: value.g,
//             b: value.b,
//             a: 255,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub(crate) struct Color {
//     g: u8,
//     b: u8,
// }

// impl Color {
//     pub(crate) fn new_rand(rng: &mut impl Rng) -> Color {
//         Color {
//             g: rng.gen_range(0..=255),
//             b: rng.gen_range(0..=255),
//         }
//     }

//     pub(crate) fn distance(a: &Color, b: &Color) -> f32 {
//         ((a.g as f32 - b.g as f32).powi(2)
//             + (a.b as f32 - b.b as f32).powi(2))
//         .sqrt()

//         // (a.r as f32 - b.r as f32)
//         //     .abs()
//         //     .max((a.g as f32 - b.g as f32).abs())
//         //     .max((a.b as f32 - b.b as f32).abs())

//         // (a.r as f32 - b.r as f32)
//         //     .abs()
//         //     .min((a.g as f32 - b.g as f32).abs())
//         //     .min((a.b as f32 - b.b as f32).abs())

//         // (a.r as f32 - b.r as f32).abs() + (a.g as f32 - b.g as f32).abs() + (a.b as f32 - b.b as f32).abs()
//     }
// }

// impl From<Color> for raylib::prelude::Color {
//     fn from(value: Color) -> Self {
//         raylib::prelude::Color {
//             r: 0,
//             g: value.g,
//             b: value.b,
//             a: 255,
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color {
    // b: u8,
    pub(crate) b: f32,
}

impl Color {
    pub(crate) fn new_rand(rng: &mut impl Rng) -> Color {
        // Color { b: rng.gen_range(0..=255)}
        Color { b: rng.gen_range(0.0..1.0)}
    }

    #[inline(always)]
    pub(crate) fn distance(a: &Color, b: &Color) -> f32 {
        // (a.b as f32 - b.b as f32).abs()
        (a.b - b.b).abs()
    }
}

impl From<Color> for raylib::prelude::Color {
    fn from(value: Color) -> Self {
        raylib::prelude::Color {
            r: 0,
            g: 0,
            b: (value.b * 255.0) as u8,
            a: 255,
        }
    }
}
