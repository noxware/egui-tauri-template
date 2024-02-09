use std::{cell::RefCell, rc::Rc};

use serde::Serialize;
use serde_wasm_bindgen::to_value;

use crate::tauri::invoke;

#[derive(Serialize)]
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
            ui.heading("Example");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.greet_name);
                if ui.button("Greet").clicked() {
                    let args = to_value(&GreetArgs {
                        name: self.greet_name.clone(),
                    })
                    .unwrap();
                    let output = self.greet_output.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match invoke("greet", args).await {
                            Ok(response) => {
                                *output.borrow_mut() =
                                    format!("Response: {}", response.as_string().unwrap());
                            }
                            Err(error) => {
                                *output.borrow_mut() =
                                    format!("Error: {}", error.as_string().unwrap());
                            }
                        }
                    });
                }
            });

            ui.add_space(16.0);

            if self.greet_output.borrow().is_empty() {
                ui.label("You haven't been greet yet");
            } else {
                ui.label(&*self.greet_output.borrow());
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });
            });
        });
    }
}
