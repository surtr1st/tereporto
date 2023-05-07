#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Storage {
    pub index: String,
    pub name: String,
    pub directory: String,
    pub constraint: String
}

#[derive(Debug, Default, serde::Serialize)]
pub struct StorageBox {
    pub storage: Storage
}

impl Storage {
    pub fn serialize(self) -> String {
        let storage = StorageBox {
            storage: Storage {
                index: self.index,
                name: self.name,
                directory: self.directory,
                constraint: self.constraint
            },
        };
        toml::to_string_pretty(&storage).unwrap()
    }
}
