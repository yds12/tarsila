use crate::{graphics, Resources};
use lapix::{Point, Tool};
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct MouseManager {
    cursors: CursorSet,
    cursor: CursorType,
    selected_tool: Tool,
    is_on_canvas: bool,
}

impl MouseManager {
    pub fn new() -> Self {
        Self {
            cursors: CursorSet::new(),
            cursor: CursorType::Tool(Tool::Brush),
            selected_tool: Tool::Brush,
            is_on_canvas: false,
        }
    }

    pub fn sync(&mut self, is_on_canvas: bool, selected_tool: Tool) {
        self.is_on_canvas = is_on_canvas;

        if self.selected_tool != selected_tool {
            self.set_cursor(CursorType::Tool(selected_tool));
            self.selected_tool = selected_tool;
        }
    }

    pub fn cursor(&self) -> CursorType {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: CursorType) {
        self.cursor = cursor;
    }

    pub fn draw(&self) {
        if let Some(cursor) = self.cursors.0.get(&self.cursor) {
            if self.is_on_canvas {
                cursor.draw();
            }
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorType {
    Tool(Tool),
    Pan,
    Cross,
}

pub struct CursorSet(HashMap<CursorType, Cursor>);

impl CursorSet {
    pub fn new() -> Self {
        let tools = [
            (Tool::Brush, (0., -16.).into()),
            (Tool::Bucket, (0., -13.).into()),
            (Tool::Eraser, (0., -16.).into()),
            (Tool::Eyedropper, (0., -16.).into()),
            (Tool::Line, (0., -16.).into()),
            (Tool::Selection, (0., 0.).into()),
            (Tool::Move, (-8., -8.).into()),
            (Tool::Rectangle, (0., -16.).into()),
        ];

        let mut hm: HashMap<_, _> = tools
            .iter()
            .map(|(t, offset)| {
                (
                    CursorType::Tool(*t),
                    Cursor::new(CursorType::Tool(*t), *offset),
                )
            })
            .collect();

        hm.insert(
            CursorType::Pan,
            Cursor::new(CursorType::Pan, (0., 0.).into()),
        );
        hm.insert(
            CursorType::Cross,
            Cursor::new(CursorType::Cross, (-7., -7.).into()),
        );

        Self(hm)
    }
}

pub struct Cursor {
    texture: Texture2D,
    offset: Point<f32>,
}

impl Cursor {
    pub fn new(typ: CursorType, offset: Point<f32>) -> Self {
        let bytes = Resources::cursor(typ);
        let texture = Texture2D::from_file_with_format(bytes, None);

        Self { texture, offset }
    }

    pub fn draw(&self) {
        let (x, y) = mouse_position();
        graphics::draw_texture_helper(
            self.texture,
            (x + self.offset.x, y + self.offset.y).into(),
            1.,
        )
    }
}
