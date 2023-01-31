use std::path::PathBuf;
pub use crate::{Color, Bitmap, Tool, CanvasEffect};

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
    Bucket(u16, u16),
    Erase(u16, u16),
    LineStart(u16, u16),
    LineEnd(u16, u16),
    Undo,
}

impl<IMG: Bitmap> Clone for Event<IMG> {
    fn clone(&self) -> Self {
        match self {
            Self::ClearCanvas => Self::ClearCanvas,
            Self::ResizeCanvas(x, y) => Self::ResizeCanvas(*x, *y),
            Self::BrushStart => Self::BrushStart,
            Self::BrushStroke(x, y) => Self::BrushStroke(*x, *y),
            Self::BrushEnd => Self::BrushEnd,
            Self::SetTool(t) => Self::SetTool(*t),
            Self::SetMainColor(c) => Self::SetMainColor(*c),
            Self::Save(path) => Self::Save(path.clone()),
            Self::Bucket(x, y) => Self::Bucket(*x, *y),
            Self::Erase(x, y) => Self::Erase(*x, *y),
            Self::LineStart(x, y) => Self::LineStart(*x, *y),
            Self::LineEnd(x, y) => Self::LineEnd(*x, *y),
            Self::Undo => Self::Undo,
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
            Self::ResizeCanvas(_, _) => CanvasEffect::New,
            _ => CanvasEffect::None,
        }
    }
    pub fn undoable(&self) -> bool {
        match self {
            Self::ClearCanvas |
            Self::ResizeCanvas(_, _) |
            Self::BrushStart |
            Self::BrushStroke(_, _) |
            Self::BrushEnd |
            Self::SetMainColor(_) |
            Self::Bucket(_, _) |
            Self::Erase(_, _) |
            Self::LineStart(_, _) |
            Self::LineEnd(_, _) => true,
            _ => false
        }
    }
}
