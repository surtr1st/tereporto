use std::{env, fs, path::Path};

const STORE: &str = ".tereporto";

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
        let default_path = format!("{}/{}", Self::get_root_directory(), STORE);

        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
        }

        Base {
            directory: default_path,
            recursive: None,
        }
    }

    fn create_recursive(&mut self, dir: &str) -> &mut Base {
        if !Path::new(dir).is_dir() {
            fs::create_dir(dir).unwrap();
        }
        self
    }

    fn get_recursive(&mut self, dir: &str) -> &mut Base {
        self.recursive = Some(dir.to_string());
        self
    }

    fn get_root_directory() -> String {
        env::var("HOME").unwrap()
    }

    fn get_base_directory(&self) -> String {
        if let Some(recursive) = &self.recursive {
            return format!("{}/{}", self.directory, recursive);
        }
        self.directory.to_string()
    }
}
