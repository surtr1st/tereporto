use std::fs::{create_dir, metadata, File};
use std::{env, fs};

const BASE_FILENAME: &str = "links.toml";

#[derive(Debug, Default)]
pub struct TOMLHandler {
    pub directory: String,
    pub filename: String,
}

impl TOMLHandler {
    pub fn new() -> Self {
        let root = env::var("HOME").unwrap();
        let store_folder = ".tereporto";
        let default_path = format!("{}/{}", &root, &store_folder);
        let default_file = format!("{}/{}", &default_path, &BASE_FILENAME);
        let folder_existed = metadata(&default_path).unwrap().is_dir();
        let file_existed = File::open(&default_file).is_ok();
        if !folder_existed {
            create_dir(&default_path).unwrap();
        }
        if !file_existed {
            File::create(&default_file).unwrap();
        }
        TOMLHandler {
            directory: default_path,
            filename: BASE_FILENAME.to_string(),
        }
    }

    pub fn read_from_file(self) -> toml::Value {
        let file = format!("{}/{}", self.directory, self.filename);
        let content = fs::read_to_string(file).unwrap();
        content.parse::<toml::Value>().unwrap()
    }

    pub fn compose(self, data: String) {
        let file = format!("{}/{}", self.directory, self.filename);
        fs::write(file, data).expect("failed to write file");
    }
}
