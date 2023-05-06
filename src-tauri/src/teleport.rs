#[derive(Debug, Default, serde::Serialize)]
pub struct Teleport {
    pub name: String,
    pub directories: Vec<String>
}

#[derive(Debug, Default, serde::Serialize)]
pub struct TeleportBox {
    pub teleports: Teleport
}

impl Teleport {
    pub fn create(self) -> String {
        let directories = self.directories
            .clone()
            .into_iter()
            .map(toml::Value::String)
            .collect::<Vec<_>>();
        let inline_table = {
            let mut table = toml::value::Table::new();
            table.insert("name".to_string(), toml::Value::String(self.name.clone()));
            table.insert("directories".to_string(), toml::Value::Array(directories));
            table
        };

        // Create a top-level table and insert the inline table
        let mut table = toml::Table::new();
        table.insert("teleports".to_string(), toml::value::Value::Table(inline_table));

        // Serialize the top-level table to TOML
        toml::to_string(&table).expect("Failed to serialize TOML")
    }
}
