use crate::Effect;
use lapix_core::Event;

pub struct LayersPanel {
    num_layers: usize,
    active_layer: usize,
    layers_vis: Vec<bool>,
    layers_alpha: Vec<String>,
}

impl LayersPanel {
    pub fn new() -> Self {
        Self {
            num_layers: 1,
            active_layer: 0,
            layers_vis: vec![true],
            layers_alpha: vec!["255".to_owned()],
        }
    }

    pub fn sync(
        &mut self,
        num_layers: usize,
        active_layer: usize,
        layers_vis: Vec<bool>,
        layers_alpha: Vec<u8>,
    ) {
        self.active_layer = active_layer;
        self.num_layers = num_layers;
        self.layers_vis = layers_vis;
        self.layers_alpha = layers_alpha.into_iter().map(|x| x.to_string()).collect();
    }

    pub fn update(&mut self, egui_ctx: &egui::Context) -> Vec<Effect> {
        let mut events = Vec::new();

        egui::Window::new("Layers").show(egui_ctx, |ui| {
            let btn = ui.button("+");
            if btn.clicked() {
                events.push(Event::NewLayerAbove.into());
            }

            ui.label("Layer / active / visible / opacity");

            for i in 0..self.num_layers {
                ui.horizontal(|ui| {
                    ui.label((i + 1).to_string());
                    if ui.radio(i == self.active_layer, "").clicked() {
                        events.push(Event::SwitchLayer(i).into());
                    }
                    if ui.radio(self.layers_vis[i], "").clicked() {
                        events.push(Event::ChangeLayerVisibility(i, !self.layers_vis[i]).into());
                    }
                    let text_edit = ui.add(
                        egui::widgets::TextEdit::singleline(&mut self.layers_alpha[i])
                            .desired_width(30.0),
                    );

                    if text_edit.changed() {
                        if let Ok(opacity) = self.layers_alpha[i].parse() {
                            events.push(Event::ChangeLayerOpacity(i, opacity).into());
                        }
                    }
                    let btn = ui.button("x");
                    if btn.clicked() {
                        events.push(Event::DeleteLayer(i).into());
                    }
                });
            }
        });

        events
    }
}
