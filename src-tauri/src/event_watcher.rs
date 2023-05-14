use crossbeam_channel::Receiver;
use notify::event::CreateKind;
use notify::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use crate::helpers::retrieve_directory_content;

pub fn watch(
    rx: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>>,
    map: Arc<Mutex<HashMap<String, String>>>
) {
    // Process events
    loop {
        match rx.lock().unwrap().recv() {
            Ok(Ok(evt)) => {
                let file = CreateKind::File;
                let folder = CreateKind::Folder;
                if evt.kind == EventKind::Create(file) || evt.kind == EventKind::Create(folder)
                {
                    let paths = evt.paths.first().unwrap();
                    let display = paths.display().to_string();
                    let hash_map = map.lock().unwrap();
                    let mut parts = display.split('/').collect::<Vec<_>>();
                    parts.pop().unwrap();
                    let target_dir = parts.join("/");

                    if hash_map.contains_key(&target_dir) {
                        let dest = hash_map.get(&target_dir).unwrap();
                        let target_teleport_dir = retrieve_directory_content(&target_dir);
                        // Use Rayon to parallelize the file transfer
                        target_teleport_dir
                            .par_iter()
                            .for_each(|file| {
                                let filename = file.file_name().unwrap_or_else(|| {
                                    panic!("should return file: {}", file.to_str().unwrap())
                                });
                                let destination = format!("{}/{}", dest, filename.to_str().unwrap());
                                fs::rename(file, &destination).unwrap_or_else(|_| {
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
