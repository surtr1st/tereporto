use crate::hash_handler::HashHandler;
use clap::Args;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Storage {
    pub index: String,
    pub name: String,
    pub directory: String,
    pub constraint: Option<String>,
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
}

#[derive(Debug, Clone)]
pub struct NewStorage<'ns> {
    pub name: &'ns str,
    pub directory: &'ns str,
    pub constraint: Option<String>,
}

impl Storage {
    pub fn serialize(s: NewStorage) -> String {
        let storage = StorageBox {
            storage: Storage {
                index: HashHandler::encrypt(s.name),
                name: s.name.to_string(),
                directory: s.directory.to_string(),
                constraint: s.constraint,
            },
        };
        toml::to_string_pretty(&storage).unwrap()
    }
}
