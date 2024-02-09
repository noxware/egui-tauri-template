use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::tauri::invoke;

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    name: String,
}

pub struct TemplateApp {
    greet_name: String,
    greet_output: Rc<RefCell<String>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            greet_name: "Kagome Higurashi".to_owned(),
            greet_output: Rc::new(RefCell::new("".to_owned())),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Examples");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.greet_name);
                if ui.button("Send").clicked() {
                    let args = to_value(&GreetArgs {
                        name: self.greet_name.clone(),
                    })
                    .unwrap();
                    let output = self.greet_output.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let response: String = invoke("greet", args).await.as_string().unwrap();
                        *output.borrow_mut() = response;
                    });
                }
            });

            if self.greet_output.borrow().is_empty() {
                ui.label("Response: Not received yet");
            } else {
                ui.label(format!("Response: {}", self.greet_output.borrow()));
            }

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(", ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(" and ");
                    ui.hyperlink_to("tauri", "https://tauri.studio");
                    ui.label(".");
                    ui.spacing();
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
