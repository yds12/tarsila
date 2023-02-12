pub use crate::{Bitmap, CanvasEffect, Color, Point, Position, Size, Tool};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ClearCanvas,
    ResizeCanvas(Size<i32>),
    BrushStart,
    BrushStroke(Position<i32>),
    BrushEnd,
    SetTool(Tool),
    SetMainColor(Color),
    AddToPalette(Color),
    RemoveFromPalette(Color),
    Save(PathBuf),
    OpenFile(PathBuf),
    LoadPalette(PathBuf),
    Bucket(Point<i32>),
    EraseStart,
    EraseEnd,
    Erase(Position<i32>),
    LineStart(Point<i32>),
    LineEnd(Point<i32>),
    RectStart(Point<i32>),
    RectEnd(Point<i32>),
    NewLayerAbove,
    NewLayerBelow,
    SwitchLayer(usize),
    ChangeLayerVisibility(usize, bool),
    ChangeLayerOpacity(usize, u8),
    DeleteLayer(usize),
    SetSpritesheet(Size<u8>),
    StartSelection(Point<i32>),
    EndSelection(Point<i32>),
    ClearSelection,
    DeleteSelection,
    MoveStart(Point<i32>),
    MoveEnd(Point<i32>),
    Copy,
    Paste(Point<i32>),
    FlipHorizontal,
    FlipVertical,
    Undo,
}

impl Event {
    // TODO: maybe this should return a vec of fx, not a single one
    pub fn canvas_effect(&self) -> CanvasEffect {
        match self {
            Self::ClearCanvas
            | Self::DeleteSelection
            | Self::BrushStart
            | Self::BrushStroke(_)
            | Self::LineEnd(_)
            | Self::RectEnd(_)
            | Self::Bucket(_)
            | Self::MoveStart(_)
            | Self::MoveEnd(_)
            | Self::Paste(_)
            | Self::FlipHorizontal
            | Self::FlipVertical
            | Self::Erase(_) => CanvasEffect::Update,
            Self::ResizeCanvas(_) | Self::OpenFile(_) => CanvasEffect::New,
            Self::NewLayerAbove | Self::NewLayerBelow | Self::DeleteLayer(_) => CanvasEffect::Layer,
            x if x.triggers_anchoring() => CanvasEffect::Update,
            _ => CanvasEffect::None,
        }
    }

    pub fn is_drawing_event(&self) -> bool {
        match self {
            Self::BrushStart
            | Self::DeleteSelection
            | Self::BrushStroke(_)
            | Self::LineStart(_)
            | Self::LineEnd(_)
            | Self::RectStart(_)
            | Self::RectEnd(_)
            | Self::Bucket(_)
            | Self::MoveStart(_)
            | Self::MoveEnd(_)
            | Self::Paste(_)
            | Self::Erase(_) => true,
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
            | Self::ResizeCanvas(_)
            | Self::BrushStart
            | Self::BrushStroke(_)
            | Self::BrushEnd
            | Self::SetMainColor(_)
            | Self::AddToPalette(_)
            | Self::RemoveFromPalette(_)
            | Self::Bucket(_)
            | Self::Erase(_)
            | Self::LineStart(_)
            | Self::LineEnd(_)
            | Self::RectStart(_)
            | Self::RectEnd(_)
            | Self::NewLayerAbove
            | Self::NewLayerBelow
            | Self::FlipHorizontal
            | Self::FlipVertical
            | Self::SwitchLayer(_)
            | Self::ChangeLayerVisibility(_, _)
            | Self::ChangeLayerOpacity(_, _)
            | Self::DeleteLayer(_)
            | Self::MoveStart(_)
            | Self::MoveEnd(_)
            | Self::StartSelection(_)
            | Self::EndSelection(_)
            | Self::Paste(_)
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
            Self::MoveStart(_)
            | Self::MoveEnd(_)
            | Self::Copy
            | Self::LineEnd(_)
            | Self::RectEnd(_)
            | Self::FlipHorizontal
            | Self::FlipVertical
            | Self::DeleteSelection => false,
            _ => true,
        }
    }
}
