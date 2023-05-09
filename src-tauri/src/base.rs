use std::{env, fs, path::Path};

const STORE: &str = ".tereporto";

#[derive(Debug, Clone)]
pub struct Base {
    pub directory: String,
}

pub trait DirectoryControl {
    fn init_path() -> Self;
    fn get_root_directory() -> String;
    fn get_base_directory(self) -> String;
}

impl DirectoryControl for Base {
    fn init_path() -> Self {
        let default_path = format!("{}/{}", Self::get_root_directory(), STORE);

        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
        }

        Base {
            directory: default_path,
        }
    }

    fn get_root_directory() -> String {
        env::var("HOME").unwrap()
    }

    fn get_base_directory(self) -> String {
        self.directory
    }
}
