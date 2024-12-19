mod test;
mod card;
mod click;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust 111!", name)
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}
#[tauri::command]
fn login(user: String, password: String) -> Result<String, String> {
    if user == "tauri" && password == "tauri" {
        // resolve
        Ok("logged_in".to_string())
    } else {
        // reject
        Err("invalid credentials".to_string())
    }
}

use serde::Serialize;
#[derive(Serialize)]
struct Response {
    status: String,
    message: String,
}

#[tauri::command]
fn get_data(name: String) -> Result<Response, String> {
    if name == "tauri" {
        // resolve
        Ok(Response {
            status: "success".to_string(),
            message: "Hello, from Rust!".to_string(),
        })
    } else {
        // reject
        Err("invalid credentials".to_string())
    }
}

#[tauri::command]
fn get_card_name(name: String) -> Result<card::Card, String> {
    Ok(card::get_card_name(name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,my_custom_command,login,get_data,get_card_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
