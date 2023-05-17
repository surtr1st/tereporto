// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod base;
mod constants;
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
use event_watcher::watch;
use helpers::{open_selected_directory, remove_quotes};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use settings::{SystemSettings, SystemSettingsTrait};
use settings_cmd::{load_settings, save_settings};
use storage::Storage;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use storage_cmd::{create_storage, get_storages, remove_storage, update_storage};
use teleport::{Teleport, TeleportTarget};
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

    tauri::async_runtime::spawn(async move {
        let map = Arc::new(Mutex::new(HashMap::<String, String>::default()));
        loop {
            let connection = receive_connection();
            if !connection.is_empty() {
                let mut threads = vec![];
                // Handle each watcher
                for c in connection {
                    let tx_clone = tx.clone();
                    let rx_clone = Arc::clone(&rx_arc);
                    let target = remove_quotes(&c.target);
                    let destination = remove_quotes(&c.destination);

                    let map_clone = Arc::clone(&map);
                    map_clone
                        .lock()
                        .unwrap()
                        .insert(target.clone(), destination.clone());

                    let thread_watcher = std::thread::spawn(move || {
                        let tx = tx_clone;
                        let rx = rx_clone;
                        let target = target;
                        let map_inner_clone = Arc::clone(&map_clone);

                        let config = Config::default()
                            .with_poll_interval(Duration::from_secs(2))
                            .with_compare_contents(true);
                        // Create a file system watcher
                        let mut watcher: RecommendedWatcher = Watcher::new(tx, config).unwrap();

                        watcher
                            .watch(&PathBuf::from(&target), RecursiveMode::Recursive)
                            .unwrap();

                        std::thread::sleep(std::time::Duration::from_secs(1));
                        watch(rx, map_inner_clone);
                    });
                    threads.push(thread_watcher);
                }
            }
            // Delay or sleep for a certain period before the next iteration
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

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

fn receive_connection() -> Vec<TeleportTarget> {
    let mut connection = vec![];

    let connected_teleports = Teleport::get_connected();
    let storages = Storage::get_map();

    if connected_teleports.is_empty() {
        return connection;
    }

    // Check if connected
    for t in &connected_teleports {
        if storages.contains_key(&t.current_connect) {
            connection.push(TeleportTarget {
                target: t.directory.clone(),
                destination: storages.get(&t.current_connect).unwrap().clone()
            })
        }
    }

    connection
}
