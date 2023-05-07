#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Storage<'s> {
    pub index: &'s str,
    pub name: String,
    pub directory: String,
    pub bind: &'s str,
    pub primary: bool,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct StorageBox<'sb> {
    pub storage: Storage<'sb>,
}

impl<'s> Storage<'s> {
    pub fn serialize(self) -> String {
        let storage = StorageBox {
            storage: Storage {
                index: self.index,
                name: self.name,
                directory: self.directory,
                bind: self.bind,
                primary: self.primary,
            },
        };
        toml::to_string_pretty(&storage).unwrap()
    }
}
