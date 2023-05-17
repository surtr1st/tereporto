use std::fs;

use crate::{
    base::{Base, DirectoryControl},
    hash_handler::HashHandler,
    helpers::{Constraint, TELEPORT_ARCHIVE_FOLDER},
    toml_handler::TOMLHandler,
};
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
    pub directory: String,
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

#[derive(Debug, Clone, Args, serde::Serialize, serde::Deserialize)]
pub struct TeleportTarget {
    pub target: String,
    pub destination: String,
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

    pub fn get_connected() -> Vec<Constraint> {
        let mut teleports: Vec<Constraint> = vec![];
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path()
            .get_recursive(TELEPORT_ARCHIVE_FOLDER)
            .get_base_directory();

        'find_connected_teleport: for file in fs::read_dir(&dir).unwrap() {
            let entry = file.unwrap();
            let filename = entry.path().display().to_string();
            let content = handler.retrieve(&filename).read_content();

            let section = content.get("teleports");
            if let Some(teleport) = section {
                if let Some(t) = teleport.as_table() {
                    let constraint = t.get("to");
                    if constraint.is_none() {
                        continue 'find_connected_teleport;
                    }
                    if constraint.is_some() {
                        let valid_to = constraint.map(|value| value.to_string());
                        teleports.push(Constraint {
                            current_connect: valid_to.unwrap().to_string(),
                            directory: t
                                .get("directories")
                                .unwrap()
                                .as_array()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .to_string(),
                        });
                    }
                }
            }
        }

        teleports
    }
}
