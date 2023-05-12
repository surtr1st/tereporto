use crate::base::{Base, DirectoryControl};
use crate::hash_handler::HashHandler;
use crate::helpers::TELEPORT_ARCHIVE_FOLDER;
use crate::teleport::{NewTeleport, Teleport, TeleportArgs};
use crate::toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs};
use std::fs;

#[tauri::command]
pub fn get_teleports() -> Vec<Teleport> {
    let mut handler = TOMLHandler::default();
    let mut teleports = vec![];

    let dir = Base::init_path()
        .get_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_base_directory();

    for file in fs::read_dir(dir).unwrap() {
        let entry = file.unwrap();
        let filename = entry.path().display().to_string();
        let content = handler.retrieve(&filename).read_content();

        let section = content.get("teleports");
        if section.is_none() {
            continue;
        }

        if let Some(teleport) = section {
            if let Some(t) = teleport.as_table() {
                teleports.push(Teleport {
                    index: t.get("index").unwrap().to_string(),
                    name: t.get("name").unwrap().to_string(),
                    directories: t
                        .get("directories")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.to_string())
                        .collect(),
                    to: t.get("to").map(|value| value.to_string()),
                    color: t.get("color").map(|value| value.to_string()),
                });
            }
        }
    }

    teleports
}

#[tauri::command]
pub fn create_teleport(t: TeleportArgs) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let mut base = Base::init_path();
    let dir = base
        .create_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_base_directory();

    // Hashing and take this as filename
    let hasher = HashHandler::encrypt(&t.name);

    handler
        .create_file(&dir, &hasher)
        .compose(&Teleport::serialize(NewTeleport {
            name: &t.name,
            directories: &t.directories,
            to: t.to,
            color: t.color,
        }))
}

#[tauri::command]
pub fn update_teleport(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path()
        .get_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_base_directory();
    let file = format!("{}/{}", &dir, &filename);

    let mut content = handler.retrieve(&file).read_content();

    handler.update(
        &mut content,
        TOMLUpdateArgs {
            key: "teleports",
            to: MappedField {
                field: target.field,
                value: target.value,
            },
        },
    )
}
