pub use crate::{Bitmap, CanvasEffect, Color, Tool};
use std::path::PathBuf;

#[derive(Debug)]
pub enum Event<IMG: Bitmap> {
    ClearCanvas,
    ResizeCanvas(u16, u16),
    BrushStart,
    BrushStroke(u16, u16),
    BrushEnd,
    SetTool(Tool),
    SetMainColor(IMG::Color),
    Save(PathBuf),
    OpenFile(PathBuf),
    Bucket(u16, u16),
    EraseStart,
    EraseEnd,
    Erase(u16, u16),
    LineStart(u16, u16),
    LineEnd(u16, u16),
    NewLayerAbove,
    NewLayerBelow,
    SwitchLayer(usize),
    ChangeLayerVisibility(usize, bool),
    ChangeLayerOpacity(usize, u8),
    DeleteLayer(usize),
    Undo,
}

impl<IMG: Bitmap> Clone for Event<IMG> {
    fn clone(&self) -> Self {
        match self {
            Self::ClearCanvas => Self::ClearCanvas,
            Self::EraseStart => Self::EraseStart,
            Self::EraseEnd => Self::EraseEnd,
            Self::NewLayerAbove => Self::NewLayerAbove,
            Self::NewLayerBelow => Self::NewLayerBelow,
            Self::ResizeCanvas(x, y) => Self::ResizeCanvas(*x, *y),
            Self::BrushStart => Self::BrushStart,
            Self::BrushStroke(x, y) => Self::BrushStroke(*x, *y),
            Self::BrushEnd => Self::BrushEnd,
            Self::SwitchLayer(i) => Self::SwitchLayer(*i),
            Self::DeleteLayer(i) => Self::DeleteLayer(*i),
            Self::SetTool(t) => Self::SetTool(*t),
            Self::SetMainColor(c) => Self::SetMainColor(*c),
            Self::Save(path) => Self::Save(path.clone()),
            Self::OpenFile(path) => Self::OpenFile(path.clone()),
            Self::Bucket(x, y) => Self::Bucket(*x, *y),
            Self::Erase(x, y) => Self::Erase(*x, *y),
            Self::LineStart(x, y) => Self::LineStart(*x, *y),
            Self::LineEnd(x, y) => Self::LineEnd(*x, *y),
            Self::ChangeLayerVisibility(i, b) => Self::ChangeLayerVisibility(*i, *b),
            Self::ChangeLayerOpacity(i, n) => Self::ChangeLayerOpacity(*i, *n),
            Self::Undo => Self::Undo,
        }
    }
}

impl<IMG: Bitmap> PartialEq for Event<IMG> {
    fn eq(&self, value: &Self) -> bool {
        match (self, value) {
            (Self::ClearCanvas, Self::ClearCanvas) => true,
            (Self::BrushStart, Self::BrushStart) => true,
            (Self::BrushEnd, Self::BrushEnd) => true,
            (Self::EraseStart, Self::EraseStart) => true,
            (Self::EraseEnd, Self::EraseEnd) => true,
            (Self::Undo, Self::Undo) => true,
            (Self::NewLayerAbove, Self::NewLayerAbove) => true,
            (Self::NewLayerBelow, Self::NewLayerBelow) => true,
            (Self::ResizeCanvas(x, y), Self::ResizeCanvas(i, j)) => x == i && y == j,
            (Self::BrushStroke(x, y), Self::BrushStroke(i, j)) => x == i && y == j,
            (Self::Bucket(x, y), Self::Bucket(i, j)) => x == i && y == j,
            (Self::Erase(x, y), Self::Erase(i, j)) => x == i && y == j,
            (Self::LineStart(x, y), Self::LineStart(i, j)) => x == i && y == j,
            (Self::LineEnd(x, y), Self::LineEnd(i, j)) => x == i && y == j,
            (Self::SetTool(t), Self::SetTool(u)) => t == u,
            (Self::SetMainColor(c), Self::SetMainColor(d)) => c == d,
            (Self::Save(p), Self::Save(q)) => p == q,
            (Self::OpenFile(p), Self::OpenFile(q)) => p == q,
            (Self::SwitchLayer(i), Self::SwitchLayer(j)) => i == j,
            (Self::DeleteLayer(i), Self::DeleteLayer(j)) => i == j,
            (Self::ChangeLayerVisibility(i, b), Self::ChangeLayerVisibility(j, p)) => {
                i == j && b == p
            }
            (Self::ChangeLayerOpacity(i, n), Self::ChangeLayerOpacity(j, m)) => i == j && n == m,
            _ => false,
        }
    }
}

impl<IMG: Bitmap> Event<IMG> {
    pub fn canvas_effect(&self) -> CanvasEffect {
        match self {
            Self::ClearCanvas
            | Self::BrushStart
            | Self::BrushStroke(_, _)
            | Self::LineEnd(_, _)
            | Self::Bucket(_, _)
            | Self::Erase(_, _) => CanvasEffect::Update,
            Self::ResizeCanvas(_, _) | Self::OpenFile(_) => CanvasEffect::New,
            Self::NewLayerAbove | Self::NewLayerBelow => CanvasEffect::Layer,
            _ => CanvasEffect::None,
        }
    }

    pub fn is_drawing_event(&self) -> bool {
        match self {
            Self::BrushStart
            | Self::BrushStroke(_, _)
            | Self::LineStart(_, _)
            | Self::LineEnd(_, _)
            | Self::Bucket(_, _)
            | Self::Erase(_, _) => true,
            _ => false,
        }
    }

    pub fn repeatable(&self) -> bool {
        match self {
            Self::Undo | Self::NewLayerAbove | Self::NewLayerBelow | Self::DeleteLayer(_) => true,
            _ => false,
        }
    }
    pub fn undoable(&self) -> bool {
        match self {
            Self::ClearCanvas
            | Self::ResizeCanvas(_, _)
            | Self::BrushStart
            | Self::BrushStroke(_, _)
            | Self::BrushEnd
            | Self::SetMainColor(_)
            | Self::Bucket(_, _)
            | Self::Erase(_, _)
            | Self::LineStart(_, _)
            | Self::LineEnd(_, _)
            | Self::NewLayerAbove
            | Self::NewLayerBelow
            | Self::SwitchLayer(_)
            | Self::ChangeLayerVisibility(_, _)
            | Self::ChangeLayerOpacity(_, _)
            | Self::DeleteLayer(_)
            | Self::OpenFile(_) => true,
            _ => false,
        }
    }
}
