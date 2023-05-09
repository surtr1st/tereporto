use std::{env, fs, path::Path};

const STORE: &str = ".tereporto";

pub fn get_root_directory() -> String {
    env::var("HOME").unwrap()
}

pub fn get_base_directory() -> String {
    let default_path = format!("{}/{}", get_root_directory(), STORE);

    // Create directory if it doesn't exist
    if !Path::new(&default_path).is_dir() {
        fs::create_dir(&default_path).unwrap();
    }

    default_path
}
