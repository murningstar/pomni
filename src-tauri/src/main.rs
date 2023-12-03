// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::{dialog, path}, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn mycustomcommand(asd: &str) -> String {
    return format!("the message you sent is: \"{}\"", asd);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            /* Код для получения cacheDir пользователя или для
            диалогового окна, о том, что программе не удается получить её
            от операционной системы пользователя. (Вдруг какой-то pepegalinux) */
            if let Some(mut cache_dir) = path::cache_dir() {
                cache_dir.push("selected_vault.md");
                /* Если vault выбрана, открыть main window */
                if cache_dir.exists() { 
                    if let Some(main_window) = app.get_window("main") {
                        main_window.show();
                    }
                } 
                /* Иначе, открыть vaults window */
                else { 
                    if let Some(vaults_window) = app.get_window("vaults") {
                        vaults_window.show();
                    }
                }
            } else {
                // dialog::message(parent_window, title, message)
            }
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![greet, mycustomcommand])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
