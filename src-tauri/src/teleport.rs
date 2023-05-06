#[derive(Debug)]
pub struct Teleport {
    pub name: String,
    pub directories: Vec<String>
}

impl Teleport {
    pub fn create(self) -> String {
        let mut table = toml::Table::new();
        let mut teleport_table = toml::Table::new();
        // Add key/value pairs
        let directories = self.directories
            .into_iter()
            .map(toml::Value::String)
            .collect();
        table.insert("name".to_owned(), toml::Value::String(self.name));
        table.insert("directories".to_owned(), toml::Value::Array(directories));
        // Add nested table
        teleport_table.insert("teleports".to_owned(), toml::Value::Table(table));

        // Convert the TOML table to a string
        toml::to_string(&teleport_table).expect("failed to serialize TOML")
    }
}
