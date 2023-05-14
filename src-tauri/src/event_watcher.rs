use crossbeam_channel::Receiver;
use notify::event::CreateKind;
use notify::*;
use rayon::prelude::*;
use std::fs;
use std::sync::{Arc, Mutex};

pub fn watch(
    rx: Arc<Mutex<Receiver<std::result::Result<Event, Error>>>>,
    target: &str,
    dest: &str,
) {
    // Process events
    loop {
        match rx.lock().unwrap().recv() {
            Ok(Ok(evt)) => {
                let file = CreateKind::File;
                let folder = CreateKind::Folder;
                if evt.kind == EventKind::Create(file) || evt.kind == EventKind::Create(folder)
                {
                    let paths = evt.paths;
                    for path in paths {
                        if let Some(file_name) = path.file_name() {
                            println!("File added: {:?} from {}", file_name, &target);
                        }
                        if path.is_dir() {
                            println!("Folder added: {} from {}", path.display(), &target);
                        }
                    }

                    let teleport_dir_target = fs::read_dir(target)
                        .expect("should read the directory specified!")
                        .map(|entry| entry.unwrap().path())
                        .filter(|path| path.is_file() || path.is_dir())
                        .collect::<Vec<_>>();

                    // Use Rayon to parallelize the file transfer
                    teleport_dir_target.par_iter().for_each(|file| {
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
            Ok(Err(e)) => {
                eprintln!("Received error event: {:?}", e);
            }
            Err(e) => {
                eprintln!("Error receiving event: {:?}", e);
            }
        }
    }
}
