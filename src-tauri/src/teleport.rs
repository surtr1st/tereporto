use crate::hash_handler::HashHandler;
use clap::Args;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Teleport {
    pub index: String,
    pub name: String,
    pub directories: Vec<String>,
    pub to: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct TeleportBox {
    pub teleports: Teleport,
}

#[derive(Args, serde::Serialize, serde::Deserialize)]
pub struct TeleportArgs {
    pub name: String,
    pub directories: Vec<String>,
    pub to: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewTeleport<'nt> {
    pub name: &'nt str,
    pub directories: &'nt Vec<String>,
    pub to: Option<String>,
    pub color: Option<String>,
}

impl Teleport {
    pub fn serialize(t: NewTeleport) -> String {
        let teleport = TeleportBox {
            teleports: Teleport {
                index: HashHandler::encrypt(t.name),
                name: t.name.to_string(),
                directories: t.directories.to_vec(),
                to: t.to,
                color: t.color,
            },
        };
        toml::to_string_pretty(&teleport).unwrap()
    }
}
