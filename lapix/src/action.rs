use crate::{Bitmap, CanvasEffect, Color, Layer, Layers, Point};
use std::fmt::Debug;

pub type LayerIndex = usize;

pub struct Action<IMG>(Vec<AtomicAction<IMG>>);

impl<IMG> Debug for Action<IMG> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("Action([")?;

        for action in self.0.iter() {
            f.write_fmt(format_args!("{:?}, ", action))?;
        }

        f.write_str("])")
    }
}

impl<IMG> From<Vec<AtomicAction<IMG>>> for Action<IMG> {
    fn from(actions: Vec<AtomicAction<IMG>>) -> Self {
        Self(actions)
    }
}

impl<IMG: Bitmap> Action<IMG> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, action: AtomicAction<IMG>) {
        self.0.push(action);
    }

    pub fn append(&mut self, actions: Vec<AtomicAction<IMG>>) {
        for action in actions {
            self.push(action);
        }
    }

    pub fn apply(mut self, layers: &mut Layers<IMG>) -> CanvasEffect {
        let mut effect = CanvasEffect::None;

        while let Some(action) = self.0.pop() {
            effect = action.apply(layers);
        }

        effect
    }
}

pub enum AtomicAction<IMG> {
    SetPixel(LayerIndex, Point<i32>, Color),
    DestroyLayer(LayerIndex),
    CreateLayer(LayerIndex, Layer<IMG>),
    SetLayerCanvas(LayerIndex, IMG),
}

impl<IMG> Debug for AtomicAction<IMG> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SetPixel(i, p, c) => f
                .debug_tuple("SetPixel")
                .field(&i)
                .field(&p)
                .field(&c)
                .finish(),
            Self::DestroyLayer(i) => f.debug_tuple("DestroyLayer").field(&i).finish(),
            Self::CreateLayer(i, _) => f.debug_tuple("CreateLayer").field(&i).finish(),
            Self::SetLayerCanvas(i, _) => f.debug_tuple("SetLayerCanvas").field(&i).finish(),
        }
    }
}

impl<IMG: Bitmap> AtomicAction<IMG> {
    pub fn set_pixel_vec(i: LayerIndex, values: Vec<(Point<i32>, Color)>) -> Vec<Self> {
        values
            .into_iter()
            .map(|(p, c)| AtomicAction::SetPixel(i, p, c))
            .collect()
    }

    pub fn apply(self, layers: &mut Layers<IMG>) -> CanvasEffect {
        match self {
            Self::SetPixel(i, p, color) => {
                layers.canvas_at_mut(i).set_pixel(p, color);
            }
            Self::DestroyLayer(i) => {
                layers.delete(i);
            }
            Self::CreateLayer(i, layer) => {
                layers.add_at(i, layer);
            }
            Self::SetLayerCanvas(i, img) => {
                layers.canvas_at_mut(i).set_img(img);
            }
            _ => todo!(),
        }
        CanvasEffect::Layer
    }
}
