use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tool {
    Brush,
    Eraser,
    Eyedropper,
    Bucket,
    Line,
    Selection,
    Move,
    Rectangle,
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let st = match self {
            Self::Brush => "brush",
            Self::Eraser => "eraser",
            Self::Eyedropper => "eyedropper",
            Self::Bucket => "bucket",
            Self::Line => "line",
            Self::Selection => "selection",
            Self::Move => "move",
            Self::Rectangle => "rectangle",
        };

        f.write_str(st)
    }
}

