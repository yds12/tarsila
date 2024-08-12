//! Functions that can be applied to an image, modifying it

use crate::Color;
use crate::{color, Bitmap, ColorF32};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transform {
    Identity,
    Silhouete,
    ApplyPalette,
}

impl Transform {
    pub fn apply<IMG: Bitmap>(&self, image: &mut IMG, palette: Vec<Color>) {
        match self {
            Self::Identity => (),
            Self::Silhouete => Self::silhouette(image),
            Self::ApplyPalette => Self::apply_palette(image, &palette),
        }
    }

    fn silhouette<IMG: Bitmap>(image: &mut IMG) {
        for i in 0..image.width() {
            for j in 0..image.height() {
                let p = (i, j).into();
                let color = image.pixel(p);
                if color.a > 127 {
                    image.set_pixel(p, color::BLACK);
                }
            }
        }
    }

    fn apply_palette<IMG: Bitmap>(image: &mut IMG, palette: &[Color]) {
        for i in 0..image.width() {
            for j in 0..image.height() {
                let p = (i, j).into();
                let color = image.pixel(p);

                let mut min_dist = f32::MAX;
                let mut min_index = 0;
                for (i, palette_color) in palette.iter().enumerate() {
                    let colorf: ColorF32 = (*palette_color).into();
                    let dist = colorf.dist(&color.into());

                    if dist < min_dist {
                        min_dist = dist;
                        min_index = i;
                    }
                }

                image.set_pixel(p, palette[min_index]);
            }
        }
    }
}
