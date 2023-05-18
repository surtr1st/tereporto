use std::{env, fs, path::Path};

use crate::constants::MAIN_ARCHIVE;

#[derive(Debug, Clone)]
pub struct Base {
    pub directory: String,
    pub recursive: Option<String>,
}

pub trait DirectoryControl {
    fn init_path() -> Self;
    fn create_recursive(&mut self, dir: &str) -> &mut Base;
    fn get_recursive(&mut self, dir: &str) -> &mut Base;
    fn get_root_directory() -> String;
    fn get_base_directory(&self) -> String;
}

impl DirectoryControl for Base {
    fn init_path() -> Self {
        let default_path = format!("{}/{}", Self::get_root_directory(), MAIN_ARCHIVE);

        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
        }

        Base {
            directory: default_path,
            recursive: None,
        }
    }

    fn create_recursive(&mut self, dir: &str) -> &mut Base {
        let directory = format!("{}/{}", self.directory, dir);
        if !Path::new(&directory).is_dir() {
            fs::create_dir(directory).unwrap();
        }
        self
    }

    fn get_recursive(&mut self, dir: &str) -> &mut Base {
        let directory = format!("{}/{}", self.directory, dir);
        if Path::new(&directory).is_dir() {
            self.recursive = Some(dir.to_string());
        }
        self
    }

    fn get_root_directory() -> String {
        let current_os = env::consts::OS;
        if current_os == "windows" {
            return env::var("USERPROFILE").unwrap();
        }
        env::var("HOME").unwrap()
    }

    fn get_base_directory(&self) -> String {
        if let Some(recursive) = &self.recursive {
            return format!("{}/{}", self.directory, recursive);
        }
        self.directory.to_string()
    }
}
