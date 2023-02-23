use crate::{color, Bitmap};

pub enum Transform {
    Identity,
    Silhouete,
}

impl Transform {
    pub fn apply<IMG: Bitmap>(&self, image: &mut IMG) {
        match self {
            Self::Identity => (),
            Self::Silhouete => Self::silhouette(image),
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
}
