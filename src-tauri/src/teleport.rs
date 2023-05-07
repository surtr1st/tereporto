#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Teleport<'t> {
    pub index: &'t str,
    pub name: String,
    pub directories: Vec<String>,
    pub to: &'t str,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct TeleportBox<'tb> {
    pub teleports: Teleport<'tb>,
}

impl<'t> Teleport<'t> {
    pub fn serialize(self) -> String {
        let teleport = TeleportBox {
            teleports: Teleport {
                index: self.index,
                name: self.name,
                directories: self.directories,
                to: self.to,
            },
        };
        toml::to_string_pretty(&teleport).unwrap()
    }
}
