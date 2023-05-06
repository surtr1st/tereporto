#[derive(Debug)]
pub struct Storage {
    pub name: String,
    pub directories: Vec<StorageDirectory>
}

#[derive(Debug)]
pub struct StorageDirectory {
    pub dir: String,
    pub primary: bool
}

impl Storage {
    pub fn create(self) -> String {
        let mut table = toml::Table::new();
        let mut dir_table = toml::Table::new();
        let mut storage_table = toml::Table::new();
        // Add key/value pairs
        for dir in self.directories {
            dir_table.insert("dir".to_owned(), toml::Value::String(dir.dir));
            dir_table.insert("primary".to_owned(), toml::Value::Boolean(true));
        }
        // Add nested table
        table.insert("name".to_owned(), toml::Value::String(self.name));
        table.insert("directories".to_owned(), toml::Value::Table(dir_table));
        storage_table.insert("storage".to_owned(), toml::Value::Table(table));

        // Convert the TOML table to a string
        toml::to_string(&storage_table).expect("failed to serialize TOML")
    }
}
