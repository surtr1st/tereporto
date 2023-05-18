// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod base;
mod constants;
mod event;
mod event_watcher;
mod hash_handler;
mod helpers;
mod settings;
mod settings_cmd;
mod storage;
mod storage_cmd;
mod teleport;
mod teleport_cmd;
mod toml_handler;

use base::{Base, DirectoryControl};
use crossbeam_channel::unbounded;
use event::on_teleport_event;
use helpers::open_selected_directory;
use settings::{SystemSettings, SystemSettingsTrait};
use settings_cmd::{load_settings, save_settings};
use std::sync::Arc;
use std::sync::Mutex;
use storage_cmd::{create_storage, get_storages, remove_storage, update_storage};
use teleport_cmd::{create_teleport, get_teleports, remove_teleport, update_teleport};

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

fn main() {
    Base::init_path();
    // Create a channel to receive events
    let (tx, rx) = unbounded();
    let rx_arc = Arc::new(Mutex::new(rx));

    on_teleport_event(tx, rx_arc);

    tauri::Builder::default()
        .system_tray(create_system_tray())
        .on_system_tray_event(handle_system_tray)
        .on_window_event(SystemSettings::prevent_frontend_on_close)
        .invoke_handler(tauri::generate_handler![
            get_storages,
            create_storage,
            update_storage,
            get_teleports,
            create_teleport,
            update_teleport,
            open_selected_directory,
            remove_teleport,
            remove_storage,
            load_settings,
            save_settings
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(SystemSettings::prevent_backend_on_close)
}

fn create_system_tray() -> SystemTray {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn handle_system_tray(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "open" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
