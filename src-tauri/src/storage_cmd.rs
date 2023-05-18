use crate::base::{Base, DirectoryControl};
use crate::constants::{
    STORAGE_ARCHIVE_FOLDER, STORAGE_KEY, TELEPORT_ARCHIVE_FOLDER, TELEPORT_KEY,
};
use crate::hash_handler::HashHandler;
use crate::helpers::{remove_quotes, retrieve_directory_files};
use crate::storage::{NewStorage, Storage, StorageArgs};
use crate::toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs};

#[tauri::command]
pub fn get_storages() -> Vec<Storage> {
    let mut handler = TOMLHandler::default();
    let mut storages = vec![];

    let dir = Base::init_path()
        .get_recursive(STORAGE_ARCHIVE_FOLDER)
        .get_base_directory();

    let files: Vec<_> = retrieve_directory_files(&dir);

    for file in files {
        let filename = file.path().display().to_string();
        let content = handler.retrieve(&filename).read_content();

        let section = handler.get_content(&content, STORAGE_KEY);
        if let Ok(s) = section {
            storages.push(Storage {
                index: s.get("index").unwrap().to_string(),
                name: s.get("name").unwrap().to_string(),
                directory: s.get("directory").unwrap().to_string(),
                constraint: s.get("constraint").map(|value| value.to_string()),
                color: s.get("color").map(|value| value.to_string()),
            });
        }
    }

    storages
}

#[tauri::command]
pub fn create_storage(storages: Vec<StorageArgs>) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let mut base = Base::init_path();
    let dir = base
        .create_recursive(STORAGE_ARCHIVE_FOLDER)
        .get_recursive(STORAGE_ARCHIVE_FOLDER)
        .get_base_directory();

    storages.iter().for_each(|s| {
        // Hashing and take this as filename
        let hasher = HashHandler::encrypt(&s.name);
        handler
            .create_file(&dir, &hasher)
            .compose(&Storage::serialize(NewStorage {
                name: &s.name,
                directory: &s.directory,
                constraint: s.constraint.clone(),
                color: s.color.clone(),
            }))
            .unwrap();
    });

    Ok(String::from("Created storage successfully!"))
}

#[tauri::command]
pub fn update_storage(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path()
        .get_recursive(STORAGE_ARCHIVE_FOLDER)
        .get_base_directory();
    let file = format!("{}/{}", &dir, &filename);

    let mut content = handler.retrieve(&file).read_content();

    handler.update(
        &mut content,
        TOMLUpdateArgs {
            key: STORAGE_KEY,
            to: MappedField {
                field: target.field,
                value: target.value,
            },
        },
    )
}

#[tauri::command]
pub fn remove_storage(filename: String) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let mut base = Base::init_path();

    let teleport_dir = base
        .get_recursive(TELEPORT_ARCHIVE_FOLDER)
        .get_base_directory();
    let dir = base
        .get_recursive(STORAGE_ARCHIVE_FOLDER)
        .get_base_directory();

    let files: Vec<_> = retrieve_directory_files(&teleport_dir);

    for file in files {
        let file_in_teleport = file.path().display().to_string();
        let mut content = handler.retrieve(&file_in_teleport).read_content();

        let section = handler.get_content(&content, TELEPORT_KEY);
        if let Ok(t) = section {
            let constraint_field = t.get("to");
            if constraint_field.is_none() {
                continue;
            }

            let constraint = remove_quotes(&constraint_field.unwrap().to_string());
            if *constraint == *filename {
                handler
                    .remove_field(&mut content, TELEPORT_KEY, "to")
                    .unwrap();
                handler
                    .remove_field(&mut content, TELEPORT_KEY, "color")
                    .unwrap();
            }
        }
    }

    let file = format!("{}/{}.toml", &dir, &filename);
    handler.remove(&file)
}
