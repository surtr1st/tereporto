use std::{env, fs, path::Path};

const BASE_FILENAME: &str = "links.toml";

#[derive(Debug, Default, Clone)]
pub struct TOMLHandler {
    pub directory: String,
    pub filename: String,
}

impl TOMLHandler {
    pub fn new() -> Self {
        let root = env::var("HOME").unwrap();
        let store_folder = ".tereporto";
        let default_path = format!("{}/{}", root, store_folder);
        let default_file = format!("{}/{}", default_path, BASE_FILENAME);

        // Create directory if it doesn't exist
        if !Path::new(&default_path).exists() {
            fs::create_dir(&default_path).unwrap();
        }

        // Create file if it doesn't exist
        if !Path::new(&default_file).exists() {
            fs::File::create(&default_file).unwrap();
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

    pub fn compose(self, target: String) {
        let file = format!("{}/{}", self.directory, self.filename);
        // Convert the TOML table to a string
        fs::write(file, target).expect("failed to write file");
    }

    pub fn create_unique_key(self, key: String) {
        let file = format!("{}/{}", self.directory, self.filename);
        // Create a new key and set its value
        fs::write(file, key).expect("failed to write file");
    }
}
