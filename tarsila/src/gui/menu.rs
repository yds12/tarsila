use crate::{project, Effect};
use lapix::{Color, Event, LoadProject, Position, SaveProject, Size, Tool};
use std::path::PathBuf;

pub struct MenuBar {
    last_file: Option<PathBuf>,
    show_resize_window: bool,
    show_spritesheet_window: bool,
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
        let mut events = Vec::new();

        egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    /*
                    ui.menu_button("Category", |ui| {
                        if ui.button("Item 1").clicked() {
                        }
                        if ui.button("Item 2").clicked() {
                        }
                    });*/
                    if ui.button("Resize canvas").clicked() {
                        ui.close_menu();
                        self.show_resize_window = true;
                    }
                    if ui.button("Change spritesheet").clicked() {
                        ui.close_menu();
                        self.show_spritesheet_window = true;
                    }
                    if ui.button("Erase canvas").clicked() {
                        ui.close_menu();
                        events.push(Event::ClearCanvas.into());
                    }
                    if ui.button("Save Project").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new();

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir).set_file_name("project.tarsila");
                        }

                        if let Some(path) = dialog.save_file() {
                            self.last_file = Some(path.clone());
                            events
                                .push(Event::SaveProject(path, SaveProject(project::save)).into());
                        }
                    }
                    if ui.button("Load Project").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new();

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.pick_file() {
                            self.last_file = Some(path.clone());
                            events
                                .push(Event::LoadProject(path, LoadProject(project::load)).into());
                        }
                    }
                    if ui.button("Export Image").clicked() {
                        ui.close_menu();
                        let mut dialog = rfd::FileDialog::new();

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
                        let mut dialog = rfd::FileDialog::new();

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.pick_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::OpenFile(path).into());
                            events.push(Event::SetTool(Tool::Move).into());
                        }
                    }
                });
            });
        });

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
}
