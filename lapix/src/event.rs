pub use crate::{Bitmap, CanvasEffect, Color, Tool};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ClearCanvas,
    ResizeCanvas(u16, u16),
    BrushStart,
    BrushStroke(u16, u16),
    BrushEnd,
    SetTool(Tool),
    SetMainColor(Color),
    AddToPalette(Color),
    RemoveFromPalette(Color),
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

impl Event {
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
            | Self::FlipVertical
            | Self::DeleteSelection => false,
            _ => true,
        }
    }
}
