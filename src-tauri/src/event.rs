use crate::event_watcher::{receive_connection, watch};
use crate::helpers::{convert_to_linux_path, is_windows_path, remove_quotes};
use crossbeam_channel::{Receiver, Sender};
use notify::{Config, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub fn on_teleport_event(
    tx: Sender<std::result::Result<Event, Error>>,
    rx: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>>,
) {
    tauri::async_runtime::spawn(async move {
        let map = Arc::new(Mutex::new(HashMap::<String, String>::default()));
        loop {
            let connection = receive_connection();
            if !connection.is_empty() {
                // Handle each watcher
                for c in connection {
                    let tx_clone = tx.clone();
                    let rx_clone = Arc::clone(&rx);
                    let target = remove_quotes(&c.target);
                    let destination = remove_quotes(&c.destination);
                    let map_clone = Arc::clone(&map);

                    if is_windows_path(&target) && is_windows_path(&destination) {
                        let teleport_dir = convert_to_linux_path(&target);
                        let storage_dir = convert_to_linux_path(&destination);
                        map_clone
                            .lock()
                            .unwrap()
                            .insert(teleport_dir.clone(), storage_dir.clone());
                    } else {
                        map_clone
                            .lock()
                            .unwrap()
                            .insert(target.clone(), destination.clone());
                    }

                    std::thread::spawn(move || {
                        let mut target = target;
                        let map_inner_clone = Arc::clone(&map_clone);

                        let config = Config::default()
                            .with_poll_interval(Duration::from_secs(2))
                            .with_compare_contents(true);

                        // Create a file system watcher
                        let mut watcher: RecommendedWatcher =
                            Watcher::new(tx_clone, config).unwrap();

                        if is_windows_path(&target) {
                            target = convert_to_linux_path(&target);
                        }

                        watcher
                            .watch(&PathBuf::from(&target), RecursiveMode::Recursive)
                            .unwrap();
                        watch(rx_clone, map_inner_clone);
                    });
                }
            }
            // Delay or sleep for a certain period before the next iteration
            std::thread::sleep(Duration::from_secs(1));
        }
    });
}
