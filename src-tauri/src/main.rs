// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod base;
mod event_watcher;
mod hash_handler;
mod helpers;
mod storage;
mod storage_cmd;
mod teleport;
mod teleport_cmd;
mod toml_handler;

use base::{Base, DirectoryControl};
use crossbeam_channel::unbounded;
use event_watcher::watch;
use helpers::{open_selected_directory, remove_quotes};
use notify::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use storage_cmd::{create_storage, get_storages, remove_storage, update_storage};
use teleport::{Teleport, TeleportTarget};
use teleport_cmd::{create_teleport, get_teleports, remove_teleport, update_teleport};

use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, WindowEvent,
};

fn main() {
    Base::init_path();
    // Create a channel to receive events
    let (tx, rx) = unbounded();
    let rx_arc = Arc::new(Mutex::new(rx));

    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn(async move {
                loop {
                    let connection = receive_connection();
                    println!("Watching...");
                    if !connection.is_empty() {
                        // Handle each watcher
                        for c in connection {
                            let tx_clone = tx.clone();
                            let rx_clone = Arc::clone(&rx_arc);
                            let target = remove_quotes(&c.target);
                            let destination = remove_quotes(&c.destination); 

                            std::thread::spawn(move || {
                                // Create a file system watcher
                                let config = Config::default()
                                    .with_poll_interval(Duration::from_secs(2))
                                    .with_compare_contents(true);
                                let mut watcher: RecommendedWatcher =
                                Watcher::new(tx_clone, config).unwrap();
                                watcher
                                    .watch(
                                        &PathBuf::from(&target),
                                        RecursiveMode::Recursive,
                                    )
                                    .unwrap();
                                // Start watching the folder for events
                                watch(
                                    rx_clone,
                                    &target,
                                    &destination
                                );
                            });
                        }
                    }
                    // Delay or sleep for a certain period before the next iteration
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            Ok(())
        })
        .system_tray(create_system_tray())
        .on_system_tray_event(handle_system_tray)
        .on_window_event(prevent_frontend_on_close)
        .invoke_handler(tauri::generate_handler![
            get_storages,
            create_storage,
            update_storage,
            get_teleports,
            create_teleport,
            update_teleport,
            open_selected_directory,
            remove_teleport,
            remove_storage
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

fn receive_connection() -> Vec<TeleportTarget> {
    let mut connection = vec![];

    let connected_teleports = Teleport::get_connected();
    let storages = get_storages();

    if connected_teleports.is_empty() {
        return connection;
    }

    // Check if connected
    for t in &connected_teleports {
        for s in &storages {
            if t.current_connect == s.index {
                connection.push(TeleportTarget {
                    target: t.directory.clone(),
                    destination: s.directory.clone(),
                })
            }
        }
    }

    connection
}
