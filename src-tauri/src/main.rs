#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::sync::Mutex;
use uuid::Uuid;

mod kernel;
mod security;
mod filesystem;
mod process_manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            initialize_os,
            shutdown_os
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to OrionOS", name)
}

#[tauri::command]
fn get_system_info() -> String {
    format!(
        "OrionOS v0.1.0 - A Futuristic Operating System\nSystem ID: {}\nStatus: Online",
        Uuid::new_v4()
    )
}

#[tauri::command]
fn initialize_os() -> bool {
    println!("OrionOS initializing...");
    true
}

#[tauri::command]
fn shutdown_os() -> bool {
    println!("OrionOS shutting down...");
    true
}