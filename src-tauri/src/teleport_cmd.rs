use crate::base::{Base, DirectoryControl};
use crate::constants::{TELEPORT_KEY, TELEPORT_ARCHIVE_FOLDER, STORAGE_ARCHIVE_FOLDER, STORAGE_KEY};
use crate::hash_handler::HashHandler;
use crate::helpers::remove_quotes;
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

    let entries: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .collect();

    for file in entries {
        let filename = file.path().display().to_string().clone();
        let content = handler.retrieve(&filename).read_content();

        let section = content.get(TELEPORT_KEY);
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
pub fn create_teleport(teleports: Vec<TeleportArgs>) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let mut base = Base::init_path();
    let dir = base
        .create_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_base_directory();

    teleports
        .iter()
        .for_each(|t| {
            // Hashing and take this as filename
            let hasher = HashHandler::encrypt(&t.name);
            handler
                .create_file(&dir, &hasher)
                .compose(&Teleport::serialize(NewTeleport {
                    name: &t.name,
                    directories: &vec![t.directory.clone()],
                    to: t.to.clone(),
                    color: t.color.clone(),
                })).unwrap();
        });

    Ok(String::from("Created teleport successfully!"))
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
            key: TELEPORT_KEY,
            to: MappedField {
                field: target.field,
                value: target.value,
            },
        },
    )
}

#[tauri::command]
pub fn remove_teleport(filename: String) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path()
        .get_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_base_directory();
    let storage_dir = Base::init_path()
        .get_recursive(STORAGE_ARCHIVE_FOLDER)
        .get_base_directory();

    for file in fs::read_dir(&storage_dir).unwrap() {
        let entry = file.unwrap();
        let file_in_storage = entry.path().display().to_string();
        let mut content = handler.retrieve(&file_in_storage).read_content();

        let section = content.get(STORAGE_KEY);
        if let Some(storage) = section {
            if let Some(s) = storage.as_table() {
                let constraint_field = s.get("constraint");
                if constraint_field.is_none() {
                    continue;
                }

                let constraint = remove_quotes(&constraint_field.unwrap().to_string());
                if *constraint == *filename {
                    handler
                        .remove_field(&mut content, "storage", "constraint")
                        .unwrap();
                    handler
                        .remove_field(&mut content, "storage", "color")
                        .unwrap();
                }
            }
        }
    }

    let file = format!("{}/{}.toml", &dir, &filename);
    handler.remove(&file)
}
