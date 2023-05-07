#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Teleport {
    pub index: String,
    pub name: String,
    pub directories: Vec<String>,
    pub to: String
}

#[derive(Debug, Default, serde::Serialize)]
pub struct TeleportBox {
    pub teleports: Teleport
}

impl Teleport {
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
