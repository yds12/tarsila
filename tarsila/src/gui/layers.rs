use crate::Effect;
use lapix::Event;

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

        egui::Window::new("Layers")
            .default_pos((15., 430.))
            .show(egui_ctx, |ui| {
                let btn = ui.button("+");
                if btn.clicked() {
                    events.push(Event::NewLayerAbove.into());
                    events.push(Event::SwitchLayer(self.num_layers).into());
                }

                ui.horizontal(|ui| {
                    ui.label("#");
                    ui.separator();
                    ui.label("act.");
                    ui.separator();
                    ui.label("vis.");
                    ui.separator();
                    ui.label("alpha");
                });

                for i in 0..self.num_layers {
                    ui.horizontal(|ui| {
                        ui.label((i + 1).to_string());
                        ui.separator();
                        let tooltip = format!("select layer {}", i + 1);
                        if ui
                            .radio(i == self.active_layer, "")
                            .on_hover_text(tooltip)
                            .clicked()
                        {
                            events.push(Event::SwitchLayer(i).into());
                        }
                        ui.separator();
                        let tooltip = format!("toggle visibility of layer {}", i + 1);
                        if ui
                            .radio(self.layers_vis[i], "")
                            .on_hover_text(tooltip)
                            .clicked()
                        {
                            events
                                .push(Event::ChangeLayerVisibility(i, !self.layers_vis[i]).into());
                        }
                        ui.separator();
                        let text_edit = ui.add(
                            egui::widgets::TextEdit::singleline(&mut self.layers_alpha[i])
                                .desired_width(30.0),
                        );

                        if text_edit.changed() {
                            if let Ok(opacity) = self.layers_alpha[i].parse() {
                                events.push(Event::ChangeLayerOpacity(i, opacity).into());
                            }
                        }
                        ui.set_enabled(self.num_layers > 1);
                        let btn = ui.button("x");
                        if btn.clicked() {
                            events.push(Event::DeleteLayer(i).into());

                            let select_layer = match self.active_layer {
                                x if i > x => self.active_layer,
                                x if i == x && i == 0 => 0,
                                _ => self.active_layer - 1,
                            };
                            events.push(Event::SwitchLayer(select_layer).into());
                        }
                        ui.set_enabled(true);
                    });
                }
            });

        events
    }
}
