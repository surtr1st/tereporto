use std::fs;
use crate::base::{Base, DirectoryControl};
use crate::hash_handler::HashHandler;
use crate::teleport::{NewTeleport, Teleport, TeleportArgs};
use crate::toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs};

#[tauri::command]
pub fn get_teleports() -> Vec<Teleport> {
    let mut handler = TOMLHandler::default();
    let mut teleports = vec![];

    let dir = Base::init_path().get_base_directory();
    fs::read_dir(&dir).unwrap()
        .for_each(|file| {
            let filename = file.unwrap().path().display().to_string();
            let content = handler
                .retrieve(&filename)
                .read_content();
            let teleport = Teleport {
                index: content["teleports"]["index"].to_string(),
                name: content["teleports"]["name"].to_string(),
                directories: content["teleports"]["directories"]
                    .as_table()
                    .iter()
                    .map(|dir| dir.to_string())
                    .collect(),
                to: Some(content["teleports"]["to"].to_string()) 
            };
            teleports.push(teleport);
        });

    teleports
}

#[tauri::command]
pub fn create_teleport(t: TeleportArgs) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let dir = Base::init_path().get_base_directory();

    // Hashing and take this as filename
    let hasher = HashHandler::encrypt(&t.name);

    handler
        .create_file(&dir, &hasher)
        .compose(&Teleport::serialize(NewTeleport {
            name: &t.name,
            directories: &t.directories,
            to: t.to,
        }))
}

#[tauri::command]
pub fn update_teleport(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path().get_base_directory();
    let file = format!("{}/{}", &dir, &filename);

    let mut content = handler.retrieve(&file).read_content();

    handler.update(
        &mut content,
        TOMLUpdateArgs {
            key: "teleports",
            from: MappedField {
                field: target.field,
                value: target.value,
            },
        },
    )
}
