// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod base;
mod hash_handler;
mod storage;
mod storage_cmd;
mod teleport;
mod teleport_cmd;
mod toml_handler;

use base::{Base, DirectoryControl};
use storage_cmd::{create_storage, update_storage};
use teleport_cmd::{create_teleport, update_teleport};

use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, WindowEvent,
};

fn main() {
    Base::init_path();
    tauri::Builder::default()
        .system_tray(create_system_tray())
        .on_system_tray_event(handle_system_tray)
        .on_window_event(prevent_frontend_on_close)
        .invoke_handler(tauri::generate_handler![
            create_storage,
            update_storage,
            create_teleport,
            update_teleport
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(prevent_backend_on_close)
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

fn prevent_frontend_on_close(event: GlobalWindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event.event() {
        event.window().hide().unwrap();
        api.prevent_close();
    }
}

fn prevent_backend_on_close(_app: &AppHandle, event: RunEvent) {
    if let RunEvent::ExitRequested { api, .. } = event {
        api.prevent_exit();
    }
}
