pub use crate::{Bitmap, CanvasEffect, Color, Point, Position, Size, Tool, Transform};
use std::fmt::Debug;
use std::path::PathBuf;

/// Holds a function that takes a path as input and outputs the bytes of the
/// project file found at that path.
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
/// Holds a function that takes a path and a set of bytes as input as saves
/// those bytes as a project file at that path
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

/// Represents an event. This is one of the key types of this crate. The main
/// way the [`State`] of the drawing project can be modified is by sending
/// events to it.
///
/// [`State`]: crate::State
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Erase the image of the active canvas
    ClearCanvas,
    /// Resize the whole canvas (all layers). Parts of the image that are out of
    /// bounds will be lost, and new pixels will be transparent
    ResizeCanvas(Size<i32>),
    /// This event should be triggered once the user starts drawing with the
    /// brush (i.e. mouse button is pressed)
    BrushStart,
    /// After a `BrushStart` event happens, this event should be triggered as
    /// often as possible whenever the mouse moves. It will draw a line between
    /// the last brush action position and this one. If the event is not
    /// triggered often enough, instead of smooth drawing strokes, it will draw
    /// straight lines between points.
    BrushStroke(Position<i32>),
    /// This event should be triggered when the brush drawing action is finished
    /// (i.e. mouse button is released)
    BrushEnd,
    /// Set the active tool
    SetTool(Tool),
    /// Set the main color used by most tools
    SetMainColor(Color),
    /// Add a color to the palette, if it is not already there
    AddToPalette(Color),
    /// Remove a color from the palette
    RemoveFromPalette(Color),
    /// Export the image to the defined path
    Save(PathBuf),
    /// Import an image from the defined path. The image will be loaded as a
    /// free image that can be moved around, and will resize the canvas if it's
    /// too big for it.
    OpenFile(PathBuf),
    // TODO: these should be UI events, however we need to see what to do
    // when it comes to UNDO
    /// Save the drawing project to the defined file path
    SaveProject(PathBuf, SaveProject),
    /// Load a drawing project from a path
    LoadProject(PathBuf, LoadProject),
    /// Load a palette from a file path. The file must be an image. The image
    /// will be read and colors will be added to the palette without repetition,
    /// until a certain limit of colors is reached.
    LoadPalette(PathBuf),
    /// Apply bucket to a point (fill with color)
    Bucket(Point<i32>),
    /// Similar to `BrushStart`, but for eraser
    EraseStart,
    /// Similar to `BrushEnd`, but for eraser
    EraseEnd,
    /// Similar to `BrushStroke`, but for eraser. The difference is that the
    /// eraser always uses transparent as its color
    Erase(Position<i32>),
    /// Begin a line at the specified point
    LineStart(Point<i32>),
    /// Draw a line between this point and the previous point specified with
    /// `LineStart`
    LineEnd(Point<i32>),
    /// Start drawing a rectangle at the specified point
    RectStart(Point<i32>),
    /// Draw a rectangle with corners at this point and the point specified at
    /// `RectStart`
    RectEnd(Point<i32>),
    /// Create a new layer above the current layer
    NewLayerAbove,
    /// Create a new layer below the current layer
    NewLayerBelow,
    /// Swith the active layer to the one with specified index
    SwitchLayer(usize),
    /// Make the layer specified by its index visible or invisible
    ChangeLayerVisibility(usize, bool),
    /// Change the alpha/opacity of the layer with the specified index
    ChangeLayerOpacity(usize, u8),
    /// Delete layer at index
    DeleteLayer(usize),
    /// Move the layer at specified index down (swap positions with the layer
    /// below it)
    MoveLayerDown(usize),
    /// Move the layer at specified index up (swap positions with the layer
    /// above it)
    MoveLayerUp(usize),
    /// Define how many horizontal and vertical frames this spritesheet has
    /// (default is (1, 1), that is, just one frame). This is useful for
    /// displaying animations
    SetSpritesheet(Size<u8>),
    /// Start a rectangular selection at the specified point
    StartSelection(Point<i32>),
    /// Select a rectangle with corners at this point and the point specified by
    /// `StartSelection`
    EndSelection(Point<i32>),
    /// Clear the selection
    ClearSelection,
    /// Delete the selected area or free image
    DeleteSelection,
    /// This event must be triggered when the user starts dragging the
    /// selection, specifying the initial mouse coordinate
    MoveStart(Point<i32>),
    /// Finishes moving an image. The move vector will be calculate from the
    /// initial mouse position defined in `MoveStart` and the final position
    /// defined here.
    MoveEnd(Point<i32>),
    /// Copy a selection to the clipboard
    Copy,
    /// Paste the clipboard at the specified point in the canvas
    Paste(Point<i32>),
    /// Flip the selection horizontally
    FlipHorizontal,
    /// Flip the selection vertically
    FlipVertical,
    /// Apply an image transform
    ApplyTransform(Transform),
    /// Undo the last undoable action
    Undo,
}

impl Event {
    /// Check whether two events are of the same variant
    pub fn same_variant(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    // TODO: maybe this should return a vec of fx, not a single one
    /// Returns the [`CanvasEffect`] caused by this event
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
            | Self::ApplyTransform(_)
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

    /// Whether this event can happen twice in a roll
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

    /// Whether this kind of event can happen twice in a roll
    pub fn type_repeatable(&self) -> bool {
        !matches!(
            self,
            Self::LineStart(_) | Self::LineEnd(_) | Self::RectStart(_) | Self::RectEnd(_)
        )
    }

    /// Whether this event can be undone
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
                | Self::ApplyTransform(_)
        )
    }

    /// Whether this event causes the selection to be cleared
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

    /// Whether this event causes any [`FreeImage`] to be anchored to the canvas
    ///
    /// [`FreeImage`]: crate::FreeImage
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
                | Self::ApplyTransform(_)
        )
    }
}
