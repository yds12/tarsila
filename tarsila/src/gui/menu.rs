use crate::Effect;
use lapix::{Color, Event, Position, Tool};
use std::path::PathBuf;

pub struct MenuBar {
    last_file: Option<PathBuf>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self { last_file: None }
    }

    pub fn sync(&mut self) {}

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
                    if ui.button("Erase canvas").clicked() {
                        events.push(Event::ClearCanvas.into());
                    }
                    if ui.button("Save Project").clicked() {
                        let mut dialog = rfd::FileDialog::new();

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir).set_file_name("project.tarsila");
                        }

                        if let Some(path) = dialog.save_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::SaveProject(path).into());
                        }
                    }
                    if ui.button("Load Project").clicked() {
                        let mut dialog = rfd::FileDialog::new();

                        if let Some(dir) = self.last_file.as_ref().and_then(|p| p.parent()) {
                            dialog = dialog.set_directory(dir);
                        }

                        if let Some(path) = dialog.pick_file() {
                            self.last_file = Some(path.clone());
                            events.push(Event::LoadProject(path).into());
                        }
                    }
                    if ui.button("Export Image").clicked() {
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

        events
    }
}
