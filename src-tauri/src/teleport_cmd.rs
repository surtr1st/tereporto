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
    fs::read_dir(dir).unwrap()
        .for_each(|file| {
            let filename = file.unwrap().path().display().to_string();
            let content = handler
                .retrieve(&filename)
                .read_content();
            
            let part = content.get("teleports");
            if part.is_some() {
                if let Some(teleport) = part {
                    if let Some(t) = teleport.as_table() {
                        teleports.push(Teleport {
                            index: t.get("index").unwrap().to_string(),
                            name: t.get("name").unwrap().to_string(),
                            directories: t.get("directories")
                                .unwrap()
                                .as_table()
                                .iter()
                                .map(|dir| dir.to_string())
                                .collect(),
                            to: t.get("to").is_none().then(|| String::from("")),
                            color: t.get("color").is_none().then(|| String::from(""))
                        });
                    }
                }
            }
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
            color: t.color
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
