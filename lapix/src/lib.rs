#![doc = include_str!("../README.md")]
#![allow(clippy::uninlined_format_args)]

mod action;
mod bitmap;
mod canvas;
pub mod color;
mod error;
mod event;
mod floating;
pub mod graphics;
mod layer;
mod palette;
pub mod primitives;
mod state;
mod tool;
mod transform;
mod util;

use action::{Action, AtomicAction};
pub use bitmap::Bitmap;
pub use canvas::{Canvas, CanvasEffect};
pub use color::{Color, ColorF32};
pub use error::{Error, Result};
pub use event::Event;
pub use floating::FreeImage;
pub use layer::{Layer, Layers};
use palette::Palette;
pub use primitives::*;
pub use state::{Selection, State};
pub use tool::Tool;
pub use transform::Transform;
pub use util::{LoadProject, SaveProject};

#[cfg(feature = "test-utils")]
pub use bitmap::TestImage;
