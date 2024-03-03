use std::{cell::RefCell, rc::Rc};

use hframe::Aware;
use serde::Serialize;
use serde_wasm_bindgen::to_value;

use crate::tauri::invoke;

const COUNTER_TEMPLATE: &str = r#"
<div style="display: flex; justify-content: center; align-items: center; padding: 8px; color: red; font: 36px sans-serif;">
    <span>{count}</span>
</div>
"#;

#[derive(Serialize)]
struct GreetArgs {
    name: String,
}

pub struct App {
    count: i32,
    greet_name: String,
    greet_output: Rc<RefCell<String>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            count: 0,
            greet_name: "Kagome Higurashi".to_owned(),
            greet_output: Rc::new(RefCell::new("".to_owned())),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Tauri Commands Example")
            .show(ctx, |ui| {
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
            })
            .aware();

        egui::Window::new("Devtools")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Counter controls: ");
                    if ui.button("+").clicked() {
                        self.count += 1;
                    }
                    if ui.button("-").clicked() {
                        self.count -= 1;
                    }
                });
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            })
            .aware();

        hframe::HtmlWindow::new("Web Counter")
            .content(&COUNTER_TEMPLATE.replace("{count}", &self.count.to_string()))
            .show(ctx);

        hframe::sync(ctx);
    }
}
