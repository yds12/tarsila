use crate::{Bitmap, CanvasEffect, Color, Layer, Layers, Point};
use std::fmt::Debug;

pub type LayerIndex = usize;

pub struct Action<IMG>(LayerIndex, Vec<AtomicAction<IMG>>);

impl<IMG> Debug for Action<IMG> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("Action({:?}, Vec [", self.0))?;

        for action in self.1.iter() {
            f.write_fmt(format_args!("{:?}, ", action))?;
        }

        f.write_str("])")
    }
}

impl<IMG: Bitmap> Action<IMG> {
    pub fn new(index: LayerIndex) -> Self {
        Self(index, Vec::new())
    }

    pub fn push(&mut self, action: AtomicAction<IMG>) {
        self.1.push(action);
    }

    pub fn append(&mut self, actions: Vec<AtomicAction<IMG>>) {
        for action in actions {
            self.push(action);
        }
    }

    pub fn apply(mut self, layers: &mut Layers<IMG>) -> CanvasEffect {
        let mut effect = CanvasEffect::None;

        while let Some(action) = self.1.pop() {
            effect = action.apply(self.0, layers);
        }

        effect
    }
}

pub enum AtomicAction<IMG> {
    SetPixel(Point<i32>, Color),
    DestroyLayer,
    CreateLayer(IMG),
    SetLayerCanvas(IMG),
}

impl<IMG> Debug for AtomicAction<IMG> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SetPixel(p, c) => f.debug_tuple("SetPixel").field(&p).field(&c).finish(),
            Self::DestroyLayer => f.debug_struct("DestroyLayer").finish(),
            Self::CreateLayer(_) => f.debug_struct("CreateLayer").finish(),
            Self::SetLayerCanvas(_) => f.debug_struct("SetLayerCanvas").finish(),
        }
    }
}

impl<IMG: Bitmap> AtomicAction<IMG> {
    pub fn apply(self, i: LayerIndex, layers: &mut Layers<IMG>) -> CanvasEffect {
        match self {
            Self::SetPixel(p, color) => {
                layers.canvas_at_mut(i).set_pixel(p, color);

                CanvasEffect::Update
            }
            Self::DestroyLayer => {
                layers.delete(i);

                CanvasEffect::Layer
            }
            _ => todo!(),
        }
    }
}
