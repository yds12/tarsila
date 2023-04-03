use crate::{Effect, UiEvent};
use lapix::{Event, Size, Tool, Transform};
use std::path::PathBuf;

pub struct MenuBar {
    last_file: Option<PathBuf>,
    show_resize_window: bool,
    show_spritesheet_window: bool,
    show_confirm_exit_window: bool,
    show_confirm_new_window: bool,
    canvas_size: Size<i32>,
    spritesheet: Size<u8>,
    canvas_size_str: Option<(String, String)>,
    spritesheet_str: Option<(String, String)>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            last_file: None,
            show_resize_window: false,
            show_spritesheet_window: false,
            show_confirm_exit_window: false,
            show_confirm_new_window: false,
            canvas_size: Size::ZERO,
            spritesheet: (1, 1).into(),
            canvas_size_str: None,
            spritesheet_str: None,
        }
    }

    pub fn sync(&mut self, canvas_size: Size<i32>, spritesheet: Size<u8>) {
        self.canvas_size = canvas_size;
        self.spritesheet = spritesheet;
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = self.update_menu(egui_ctx);
        events.append(&mut self.update_resize_window(egui_ctx));
        events.append(&mut self.update_spritesheet_window(egui_ctx));
        events.append(&mut self.update_confirm_exit_window(egui_ctx));
        events.append(&mut self.update_confirm_new_window(egui_ctx));
        events
    }

    fn update_menu(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.show_confirm_new_window = true;
                        ui.close_menu();
                    }
                    if ui.button("Save Project").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new()
                            .add_filter("Tarsila files", &["tarsila"])
                            .add_filter("All files", &["*"]);

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir).set_file_name("project.tarsila");
                        }

                        if let Some(path) = dialog.save_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::SaveProject(path).into());
                        }
                    }
                    if ui.button("Load Project").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new()
                            .add_filter("Tarsila files", &["tarsila"])
                            .add_filter("All files", &["*"]);

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.pick_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::LoadProject(path).into());
                        }
                    }
                    if ui.button("Export Image").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new()
                            .add_filter("PNG files", &["png"])
                            .add_filter("JPEG files", &["jpg", "jpeg"])
                            .add_filter("All files", &["*"]);

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.save_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::Save(path).into());
                        }
                    }
                    if ui.button("Import Image").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new()
                            .add_filter("All files", &["*"])
                            .add_filter("PNG files", &["png"])
                            .add_filter("JPEG files", &["jpg", "jpeg"]);

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.pick_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::OpenFile(path).into());
                            events.push(Event::SetTool(Tool::Move).into());
                        }
                    }
                    if ui.button("Exit").clicked() {
                        self.show_confirm_exit_window = true;
                        ui.close_menu();
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Zoom in").clicked() {
                        events.push(Effect::UiEvent(UiEvent::ZoomIn));
                        ui.close_menu();
                    }
                    if ui.button("Zoom out").clicked() {
                        events.push(Effect::UiEvent(UiEvent::ZoomOut));
                        ui.close_menu();
                    }
                    if ui.button("Reset zoom to default").clicked() {
                        events.push(Effect::UiEvent(UiEvent::ResetZoom));
                        ui.close_menu();
                    }
                    if ui.button("Set zoom to 100%").clicked() {
                        events.push(Effect::UiEvent(UiEvent::SetZoom100));
                        ui.close_menu();
                    }
                });
                ui.menu_button("Canvas", |ui| {
                    /*
                    ui.menu_button("Category", |ui| {
                        if ui.button("Item 1").clicked() {
                        }
                        if ui.button("Item 2").clicked() {
                        }
                    });*/
                    if ui.button("Resize Canvas").clicked() {
                        ui.close_menu();
                        self.show_resize_window = true;
                    }
                    if ui.button("Change Spritesheet").clicked() {
                        ui.close_menu();
                        self.show_spritesheet_window = true;
                    }
                    if ui.button("Erase Canvas").clicked() {
                        ui.close_menu();
                        events.push(Event::ClearCanvas.into());
                    }
                });
                ui.menu_button("Transform", |ui| {
                    if ui.button("Silhouete").clicked() {
                        ui.close_menu();
                        events.push(Event::ApplyTransform(Transform::Silhouete).into());
                    }
                });
            });
        });

        events
    }

    fn update_resize_window(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        if self.show_resize_window {
            if self.canvas_size_str.is_none() {
                self.canvas_size_str = Some((
                    self.canvas_size.x.to_string(),
                    self.canvas_size.y.to_string(),
                ));
            }

            egui::Window::new("Resize Canvas")
                .default_pos((200., 30.))
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        let label = ui.label("w:");
                        ui.add(
                            egui::widgets::TextEdit::singleline(
                                &mut self.canvas_size_str.as_mut().unwrap().0,
                            )
                            .desired_width(30.0),
                        )
                        .labelled_by(label.id);
                        let label = ui.label("h:");
                        ui.add(
                            egui::widgets::TextEdit::singleline(
                                &mut self.canvas_size_str.as_mut().unwrap().1,
                            )
                            .desired_width(30.0),
                        )
                        .labelled_by(label.id);
                    });

                    ui.horizontal(|ui| {
                        if ui.button("resize").clicked() {
                            if let (Ok(w), Ok(h)) = (
                                self.canvas_size_str.as_ref().unwrap().0.parse(),
                                self.canvas_size_str.as_ref().unwrap().1.parse(),
                            ) {
                                events.push(Event::ResizeCanvas((w, h).into()).into());
                            }
                            self.canvas_size_str = None;
                            self.show_resize_window = false;
                        }
                        if ui.button("cancel").clicked() {
                            self.canvas_size_str = None;
                            self.show_resize_window = false;
                        }
                    });
                });
        }

        events
    }

    fn update_spritesheet_window(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        if self.show_spritesheet_window {
            if self.spritesheet_str.is_none() {
                self.spritesheet_str = Some((
                    self.spritesheet.x.to_string(),
                    self.spritesheet.y.to_string(),
                ));
            }

            egui::Window::new("Spritesheet")
                .default_pos((200., 30.))
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        let label = ui.label("cols:");
                        ui.add(
                            egui::widgets::TextEdit::singleline(
                                &mut self.spritesheet_str.as_mut().unwrap().0,
                            )
                            .desired_width(30.0),
                        )
                        .labelled_by(label.id);
                        let label = ui.label("rows:");
                        ui.add(
                            egui::widgets::TextEdit::singleline(
                                &mut self.spritesheet_str.as_mut().unwrap().1,
                            )
                            .desired_width(30.0),
                        )
                        .labelled_by(label.id);
                        if ui.button("Ok").clicked() {
                            if let (Ok(w), Ok(h)) = (
                                self.spritesheet_str.as_ref().unwrap().0.parse(),
                                self.spritesheet_str.as_ref().unwrap().1.parse(),
                            ) {
                                events.push(Event::SetSpritesheet((w, h).into()).into());
                            }
                            self.spritesheet_str = None;
                            self.show_spritesheet_window = false;
                        }
                        if ui.button("cancel").clicked() {
                            self.spritesheet_str = None;
                            self.show_spritesheet_window = false;
                        }
                    });
                });
        }

        events
    }

    fn update_confirm_exit_window(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        if !self.show_confirm_exit_window {
            return events;
        }

        egui::Window::new("Exit")
            .default_pos((200., 30.))
            .show(egui_ctx, |ui| {
                ui.label("Are you sure you want to exit?");
                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() {
                        events.push(UiEvent::Exit.into());
                    }
                    if ui.button("cancel").clicked() {
                        self.show_confirm_exit_window = false;
                    }
                });
            });

        events
    }

    fn update_confirm_new_window(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        if !self.show_confirm_new_window {
            return events;
        }

        egui::Window::new("New Project")
            .default_pos((200., 30.))
            .show(egui_ctx, |ui| {
                ui.label(
                    "Are you sure you want to start a new project? \
                    All your unsaved changes will be lost",
                );
                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() {
                        events.push(UiEvent::NewProject.into());
                    }
                    if ui.button("cancel").clicked() {
                        self.show_confirm_new_window = false;
                    }
                });
            });

        events
    }
}
