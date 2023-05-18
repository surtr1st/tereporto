use std::collections::HashMap;

use crate::{
    base::{Base, DirectoryControl},
    constants::{STORAGE_ARCHIVE_FOLDER, STORAGE_KEY},
    hash_handler::HashHandler,
    helpers::retrieve_directory_files,
    toml_handler::TOMLHandler,
};
use clap::Args;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Storage {
    pub index: String,
    pub name: String,
    pub directory: String,
    pub constraint: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct StorageBox {
    pub storage: Storage,
}

#[derive(Args, serde::Serialize, serde::Deserialize)]
pub struct StorageArgs {
    pub name: String,
    pub directory: String,
    pub constraint: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewStorage<'ns> {
    pub name: &'ns str,
    pub directory: &'ns str,
    pub constraint: Option<String>,
    pub color: Option<String>,
}

impl Storage {
    pub fn serialize(s: NewStorage) -> String {
        let storage = StorageBox {
            storage: Storage {
                index: HashHandler::encrypt(s.name),
                name: s.name.to_string(),
                directory: s.directory.to_string(),
                constraint: s.constraint,
                color: s.color,
            },
        };
        toml::to_string_pretty(&storage).unwrap()
    }

    pub fn get_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path()
            .get_recursive(STORAGE_ARCHIVE_FOLDER)
            .get_base_directory();

        let files: Vec<_> = retrieve_directory_files(&dir);
        files.iter().for_each(|file| {
            let filename = file.path().display().to_string();
            let content = handler.retrieve(&filename).read_content();
            let field = handler.get_content(&content, STORAGE_KEY);
            if let Ok(f) = field {
                map.insert(
                    f.get("index").unwrap().to_string(),
                    f.get("directory").unwrap().to_string(),
                );
            }
        });

        map
    }
}
