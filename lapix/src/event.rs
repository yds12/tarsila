pub use crate::{Bitmap, CanvasEffect, Color, Point, Position, Size, Tool};
use std::fmt::Debug;
use std::path::PathBuf;

pub struct LoadProject(pub fn(PathBuf) -> Vec<u8>);
impl Debug for LoadProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("LoadProject(fn(PathBuf) -> Vec<u8>>)")
    }
}
impl PartialEq for LoadProject {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl Clone for LoadProject {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl From<fn(PathBuf) -> Vec<u8>> for LoadProject {
    fn from(val: fn(PathBuf) -> Vec<u8>) -> Self {
        Self(val)
    }
}
pub struct SaveProject(pub fn(PathBuf, Vec<u8>));
impl Debug for SaveProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("SaveProject(fn(PathBuf, Vec<u8>))")
    }
}
impl PartialEq for SaveProject {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl Clone for SaveProject {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

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
    // TODO: these should be UI events, however we need to see what to do
    // when it comes to UNDO
    SaveProject(PathBuf, SaveProject),
    LoadProject(PathBuf, LoadProject),
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
    MoveLayerDown(usize),
    MoveLayerUp(usize),
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
    pub fn same_variant(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

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
            Self::NewLayerAbove
            | Self::NewLayerBelow
            | Self::DeleteLayer(_)
            | Self::MoveLayerDown(_)
            | Self::MoveLayerUp(_)
            | Self::LoadProject(_, _) => CanvasEffect::Layer,
            x if x.triggers_anchoring() => CanvasEffect::Update,
            _ => CanvasEffect::None,
        }
    }

    pub fn is_drawing_event(&self) -> bool {
        matches!(
            self,
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
                | Self::Erase(_)
        )
    }

    pub fn repeatable(&self) -> bool {
        matches!(
            self,
            Self::Undo
                | Self::NewLayerAbove
                | Self::NewLayerBelow
                | Self::DeleteLayer(_)
                | Self::FlipHorizontal
                | Self::FlipVertical
                | Self::MoveLayerDown(_)
                | Self::MoveLayerUp(_)
        )
    }

    pub fn type_repeatable(&self) -> bool {
        !matches!(
            self,
            Self::LineStart(_) | Self::LineEnd(_) | Self::RectStart(_) | Self::RectEnd(_)
        )
    }

    pub fn undoable(&self) -> bool {
        matches!(
            self,
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
                | Self::OpenFile(_)
                | Self::MoveLayerDown(_)
                | Self::MoveLayerUp(_)
        )
    }

    pub fn clears_selection(&self) -> bool {
        matches!(
            self,
            Self::SetTool(Tool::Brush)
                | Self::ClearSelection
                | Self::DeleteSelection
                | Self::SetTool(Tool::Eyedropper)
                | Self::SetTool(Tool::Eraser)
                | Self::SetTool(Tool::Rectangle)
                | Self::SetTool(Tool::Line)
                | Self::MoveLayerDown(_)
                | Self::MoveLayerUp(_)
        )
    }

    pub fn triggers_anchoring(&self) -> bool {
        !matches!(
            self,
            Self::MoveStart(_)
                | Self::MoveEnd(_)
                | Self::SetTool(Tool::Move)
                | Self::Copy
                | Self::LineEnd(_)
                | Self::RectEnd(_)
                | Self::FlipHorizontal
                | Self::FlipVertical
                | Self::DeleteSelection
                | Self::MoveLayerDown(_)
                | Self::MoveLayerUp(_)
        )
    }
}
