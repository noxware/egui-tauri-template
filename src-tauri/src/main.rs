// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        Err("You sent an empty name to greet. Now try sending something.".to_owned())
    } else {
        Ok(format!(
            "Hello, {}! You've been greeted from Tauri! Now try to send an empty name.",
            name
        ))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
