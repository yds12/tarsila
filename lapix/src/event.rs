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
    AddToPalette(IMG::Color),
    RemoveFromPalette(IMG::Color),
    Save(PathBuf),
    OpenFile(PathBuf),
    LoadPalette(PathBuf),
    Bucket(u16, u16),
    EraseStart,
    EraseEnd,
    Erase(u16, u16),
    LineStart(u16, u16),
    LineEnd(u16, u16),
    RectStart(u16, u16),
    RectEnd(u16, u16),
    NewLayerAbove,
    NewLayerBelow,
    SwitchLayer(usize),
    ChangeLayerVisibility(usize, bool),
    ChangeLayerOpacity(usize, u8),
    DeleteLayer(usize),
    SetSpritesheet(u8, u8),
    StartSelection(u16, u16),
    EndSelection(u16, u16),
    ClearSelection,
    DeleteSelection,
    MoveStart(u16, u16),
    MoveEnd(u16, u16),
    Copy,
    Paste(u16, u16),
    FlipHorizontal,
    FlipVertical,
    Undo,
}

impl<IMG: Bitmap> Clone for Event<IMG> {
    fn clone(&self) -> Self {
        match self {
            Self::ClearCanvas => Self::ClearCanvas,
            Self::DeleteSelection => Self::DeleteSelection,
            Self::EraseStart => Self::EraseStart,
            Self::EraseEnd => Self::EraseEnd,
            Self::NewLayerAbove => Self::NewLayerAbove,
            Self::NewLayerBelow => Self::NewLayerBelow,
            Self::Copy => Self::Copy,
            Self::ClearSelection => Self::ClearSelection,
            Self::FlipHorizontal => Self::FlipHorizontal,
            Self::FlipVertical => Self::FlipVertical,
            Self::ResizeCanvas(x, y) => Self::ResizeCanvas(*x, *y),
            Self::BrushStart => Self::BrushStart,
            Self::BrushStroke(x, y) => Self::BrushStroke(*x, *y),
            Self::BrushEnd => Self::BrushEnd,
            Self::SwitchLayer(i) => Self::SwitchLayer(*i),
            Self::DeleteLayer(i) => Self::DeleteLayer(*i),
            Self::SetTool(t) => Self::SetTool(*t),
            Self::SetMainColor(c) => Self::SetMainColor(*c),
            Self::AddToPalette(c) => Self::AddToPalette(*c),
            Self::RemoveFromPalette(c) => Self::RemoveFromPalette(*c),
            Self::Save(path) => Self::Save(path.clone()),
            Self::OpenFile(path) => Self::OpenFile(path.clone()),
            Self::LoadPalette(path) => Self::LoadPalette(path.clone()),
            Self::Bucket(x, y) => Self::Bucket(*x, *y),
            Self::Erase(x, y) => Self::Erase(*x, *y),
            Self::LineStart(x, y) => Self::LineStart(*x, *y),
            Self::LineEnd(x, y) => Self::LineEnd(*x, *y),
            Self::RectStart(x, y) => Self::RectStart(*x, *y),
            Self::RectEnd(x, y) => Self::RectEnd(*x, *y),
            Self::StartSelection(x, y) => Self::StartSelection(*x, *y),
            Self::EndSelection(x, y) => Self::EndSelection(*x, *y),
            Self::MoveStart(x, y) => Self::MoveStart(*x, *y),
            Self::MoveEnd(x, y) => Self::MoveEnd(*x, *y),
            Self::Paste(x, y) => Self::Paste(*x, *y),
            Self::SetSpritesheet(x, y) => Self::SetSpritesheet(*x, *y),
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
            (Self::Copy, Self::Copy) => true,
            (Self::ClearSelection, Self::ClearSelection) => true,
            (Self::DeleteSelection, Self::DeleteSelection) => true,
            (Self::FlipHorizontal, Self::FlipHorizontal) => true,
            (Self::FlipVertical, Self::FlipVertical) => true,
            (Self::ResizeCanvas(x, y), Self::ResizeCanvas(i, j)) => x == i && y == j,
            (Self::BrushStroke(x, y), Self::BrushStroke(i, j)) => x == i && y == j,
            (Self::Bucket(x, y), Self::Bucket(i, j)) => x == i && y == j,
            (Self::Erase(x, y), Self::Erase(i, j)) => x == i && y == j,
            (Self::LineStart(x, y), Self::LineStart(i, j)) => x == i && y == j,
            (Self::LineEnd(x, y), Self::LineEnd(i, j)) => x == i && y == j,
            (Self::RectStart(x, y), Self::RectStart(i, j)) => x == i && y == j,
            (Self::RectEnd(x, y), Self::RectEnd(i, j)) => x == i && y == j,
            (Self::SetSpritesheet(x, y), Self::SetSpritesheet(i, j)) => x == i && y == j,
            (Self::StartSelection(x, y), Self::StartSelection(i, j)) => x == i && y == j,
            (Self::EndSelection(x, y), Self::EndSelection(i, j)) => x == i && y == j,
            (Self::MoveStart(x, y), Self::MoveStart(i, j)) => x == i && y == j,
            (Self::Paste(x, y), Self::Paste(i, j)) => x == i && y == j,
            (Self::SetTool(t), Self::SetTool(u)) => t == u,
            (Self::SetMainColor(c), Self::SetMainColor(d)) => c == d,
            (Self::AddToPalette(c), Self::AddToPalette(d)) => c == d,
            (Self::RemoveFromPalette(c), Self::RemoveFromPalette(d)) => c == d,
            (Self::Save(p), Self::Save(q)) => p == q,
            (Self::OpenFile(p), Self::OpenFile(q)) => p == q,
            (Self::LoadPalette(p), Self::LoadPalette(q)) => p == q,
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
    // TODO: maybe this should return a vec of fx, not a single one
    pub fn canvas_effect(&self) -> CanvasEffect {
        match self {
            Self::ClearCanvas
            | Self::DeleteSelection
            | Self::BrushStart
            | Self::BrushStroke(_, _)
            | Self::LineEnd(_, _)
            | Self::RectEnd(_, _)
            | Self::Bucket(_, _)
            | Self::MoveStart(_, _)
            | Self::MoveEnd(_, _)
            | Self::Paste(_, _)
            | Self::FlipHorizontal
            | Self::FlipVertical
            | Self::Erase(_, _) => CanvasEffect::Update,
            Self::ResizeCanvas(_, _) | Self::OpenFile(_) => CanvasEffect::New,
            Self::NewLayerAbove | Self::NewLayerBelow | Self::DeleteLayer(_) => CanvasEffect::Layer,
            x if x.triggers_anchoring() => CanvasEffect::Update,
            _ => CanvasEffect::None,
        }
    }

    pub fn is_drawing_event(&self) -> bool {
        match self {
            Self::BrushStart
            | Self::DeleteSelection
            | Self::BrushStroke(_, _)
            | Self::LineStart(_, _)
            | Self::LineEnd(_, _)
            | Self::RectStart(_, _)
            | Self::RectEnd(_, _)
            | Self::Bucket(_, _)
            | Self::MoveStart(_, _)
            | Self::MoveEnd(_, _)
            | Self::Paste(_, _)
            | Self::Erase(_, _) => true,
            _ => false,
        }
    }

    pub fn repeatable(&self) -> bool {
        match self {
            Self::Undo
            | Self::NewLayerAbove
            | Self::NewLayerBelow
            | Self::DeleteLayer(_)
            | Self::FlipHorizontal
            | Self::FlipVertical => true,
            _ => false,
        }
    }
    pub fn undoable(&self) -> bool {
        match self {
            Self::ClearCanvas
            | Self::DeleteSelection
            | Self::ResizeCanvas(_, _)
            | Self::BrushStart
            | Self::BrushStroke(_, _)
            | Self::BrushEnd
            | Self::SetMainColor(_)
            | Self::AddToPalette(_)
            | Self::RemoveFromPalette(_)
            | Self::Bucket(_, _)
            | Self::Erase(_, _)
            | Self::LineStart(_, _)
            | Self::LineEnd(_, _)
            | Self::RectStart(_, _)
            | Self::RectEnd(_, _)
            | Self::NewLayerAbove
            | Self::NewLayerBelow
            | Self::FlipHorizontal
            | Self::FlipVertical
            | Self::SwitchLayer(_)
            | Self::ChangeLayerVisibility(_, _)
            | Self::ChangeLayerOpacity(_, _)
            | Self::DeleteLayer(_)
            | Self::MoveStart(_, _)
            | Self::MoveEnd(_, _)
            | Self::StartSelection(_, _)
            | Self::EndSelection(_, _)
            | Self::Paste(_, _)
            | Self::LoadPalette(_)
            | Self::OpenFile(_) => true,
            _ => false,
        }
    }

    pub fn clears_selection(&self) -> bool {
        match self {
            Self::SetTool(Tool::Brush)
            | Self::ClearSelection
            | Self::DeleteSelection
            | Self::SetTool(Tool::Eyedropper)
            | Self::SetTool(Tool::Eraser)
            | Self::SetTool(Tool::Rectangle)
            | Self::SetTool(Tool::Line) => true,
            _ => false,
        }
    }

    pub fn triggers_anchoring(&self) -> bool {
        match self {
            Self::MoveStart(_, _)
            | Self::MoveEnd(_, _)
            | Self::Copy
            | Self::LineEnd(_, _)
            | Self::RectEnd(_, _)
            | Self::FlipHorizontal
            | Self::FlipVertical => false,
            _ => true,
        }
    }
}
