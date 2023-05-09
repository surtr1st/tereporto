use std::{env, fs, path::Path};

const STORE: &str = ".tereporto";

#[derive(Debug, Clone)]
pub struct Base {
    pub directory: String
}

pub trait DirectoryControl {
    fn init_path() -> Self;
    fn get_root_directory() -> String;
    fn get_base_directory(self) -> String;
    fn create_file(&mut self, filename: &str);
    fn create_folder(dir: &str, name: &str);
}

impl DirectoryControl for Base {
    fn init_path() -> Self {
        let default_path = format!("{}/{}", Self::get_root_directory(), STORE);

        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
        }

        Base {
            directory: default_path
        }
    }

    fn get_root_directory() -> String {
        env::var("HOME").unwrap()
    }

    fn get_base_directory(self) -> String {
        self.directory
    }

    fn create_file(&mut self, filename: &str) {
        let file = format!("{}/{}", self.directory, filename);

        // Create file if it doesn't exist
        if !Path::new(&file).is_file() {
            fs::File::create(&file).unwrap();
        }
    }

    fn create_folder(dir: &str, name: &str) {
        let folder = format!("{}/{}", dir, name);

        if !Path::new(&folder).is_dir() {
            fs::create_dir(&folder).unwrap();
        }
    }
}
