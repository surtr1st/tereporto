use crate::helpers::{is_windows_path, retrieve_directory_content};
use crate::storage::Storage;
use crate::teleport::{Teleport, TeleportTarget};
use crossbeam_channel::Receiver;
use fs_extra::file::{self, CopyOptions};
use notify::event::CreateKind;
use notify::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn watch(
    rx: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>>,
    map: Arc<Mutex<HashMap<String, String>>>,
) {
    // Process events
    loop {
        match rx.lock().unwrap().recv() {
            Ok(Ok(evt)) => {
                let any = CreateKind::Any;
                if evt.kind == EventKind::Create(any) {
                    let paths = evt.paths.first().unwrap();
                    let display = paths.display().to_string();
                    let hash_map = map.lock().unwrap();
                    let mut slash = '/';
                    if is_windows_path(&display) {
                        slash = '\\';
                    }
                    let mut parts = display.split(slash).collect::<Vec<_>>();
                    parts.pop().unwrap();
                    let target_dir = parts.join("/");

                    if hash_map.contains_key(&target_dir) {
                        let dest = hash_map.get(&target_dir).unwrap();
                        let target_teleport_dir = retrieve_directory_content(&target_dir);
                        // Use Rayon to parallelize the file transfer
                        target_teleport_dir.par_iter().for_each(|file| {
                            let filename = file.file_name().unwrap_or_else(|| {
                                panic!("should return file: {}", file.to_str().unwrap())
                            });
                            let destination = format!("{}/{}", dest, filename.to_str().unwrap());
                            let options = CopyOptions::new();
                            file::move_file(file, &destination, &options).unwrap_or_else(|_| {
                                panic!("should transfer file to {}", &destination)
                            });
                        });
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Received error event: {:?}", e);
                break;
            }
            Err(e) => {
                eprintln!("Error receiving event: {:?}", e);
                break;
            }
        }
    }
}

pub fn receive_connection() -> Vec<TeleportTarget> {
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
                destination: storages.get(&t.current_connect).unwrap().clone(),
            })
        }
    }

    connection
}
