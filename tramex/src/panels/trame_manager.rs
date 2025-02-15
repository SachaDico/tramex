use eframe::egui;
use tramex_tools::connector::Connector;
use tramex_tools::websocket::layer::Layers;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TrameManager {
    pub layers_list: Layers,
    pub should_get_more_log: bool,
}

impl TrameManager {
    pub fn new() -> Self {
        Self {
            layers_list: Layers::new(),
            should_get_more_log: false,
        }
    }
}

impl Default for TrameManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TrameManager {
    pub fn show_options(&mut self, ui: &mut egui::Ui, data: &mut Connector) {
        ui.collapsing("Options", |ui| {
            ui.horizontal(|ui| {
                ui.label("Asking size: ");
                ui.add(
                    egui::DragValue::new(&mut data.asking_size_max)
                        .speed(2.0)
                        .clamp_range(64.0..=4096.0),
                );
            });
            checkbox(ui, &mut self.layers_list.phy, "PHY");
            checkbox(ui, &mut self.layers_list.mac, "MAC");
            checkbox(ui, &mut self.layers_list.rlc, "RLC");
            checkbox(ui, &mut self.layers_list.pdcp, "PDCP");
            checkbox(ui, &mut self.layers_list.rrc, "RRC");
            checkbox(ui, &mut self.layers_list.nas, "NAS");
            checkbox(ui, &mut self.layers_list.s72, "S72");
            checkbox(ui, &mut self.layers_list.s1ap, "S1AP");
            checkbox(ui, &mut self.layers_list.ngap, "NGAP");
            checkbox(ui, &mut self.layers_list.gtpu, "GTPU");
            checkbox(ui, &mut self.layers_list.x2ap, "X2AP");
            checkbox(ui, &mut self.layers_list.xnap, "XnAP");
            checkbox(ui, &mut self.layers_list.m2ap, "M2AP");
            checkbox(ui, &mut self.layers_list.lppa, "LPPa");
            checkbox(ui, &mut self.layers_list.nrppa, "NRPPa");
            checkbox(ui, &mut self.layers_list.trx, "TRX");
        });
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui, data: &mut Connector) {
        ui.horizontal(|ui| {
            if ui.button("More").clicked() {
                log::debug!("More");
                self.should_get_more_log = true;
            }
            if ui.button("Next").clicked() {
                log::debug!("Next");
                if data.data.events.len() > data.data.current_index + 1 {
                    data.data.current_index += 1;
                } else {
                    self.should_get_more_log = true;
                }
            }
            if ui.button("Previous").clicked() {
                log::debug!("Previous");
                if data.data.current_index > 0 {
                    data.data.current_index -= 1;
                }
            }
        });
    }
}

fn checkbox(ui: &mut egui::Ui, string: &mut String, text: &str) {
    let mut checked = string == "debug";
    if ui.checkbox(&mut checked, text).changed() {
        if checked {
            *string = "debug".to_owned();
        } else {
            *string = "warn".to_owned();
        };
    };
}
