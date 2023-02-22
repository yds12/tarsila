use lapix::Tool;

pub struct Resources;

impl Resources {
    pub fn tool_icon(tool: Tool) -> &'static [u8] {
        match tool {
            Tool::Brush => include_bytes!("../res/icon/pencil.png"),
            Tool::Bucket => include_bytes!("../res/icon/bucket.png"),
            Tool::Eraser => include_bytes!("../res/icon/eraser.png"),
            Tool::Eyedropper => include_bytes!("../res/icon/eyedropper.png"),
            Tool::Line => include_bytes!("../res/icon/line.png"),
            Tool::Selection => include_bytes!("../res/icon/selection.png"),
            Tool::Move => include_bytes!("../res/icon/move.png"),
            Tool::Rectangle => include_bytes!("../res/icon/rectangle.png"),
        }
    }
}
