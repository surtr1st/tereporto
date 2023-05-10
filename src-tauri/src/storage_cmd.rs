use std::fs;
use crate::base::{Base, DirectoryControl};
use crate::hash_handler::HashHandler;
use crate::storage::{NewStorage, Storage, StorageArgs};
use crate::toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs};

#[tauri::command]
pub fn get_storages() -> Vec<Storage> {
    let mut handler = TOMLHandler::default();
    let mut storages = vec![];

    let dir = Base::init_path().get_base_directory();
    for file in fs::read_dir(dir).unwrap() {
        let filename = file.unwrap().path().display().to_string();
        let content = handler
            .retrieve(&filename)
            .read_content();

        let section = content.get("storage");
        if section.is_none() {
            continue;
        }
        if let Some(storage) = section {
            if let Some(s) = storage.as_table() {
                storages.push(Storage {
                    index: s.get("index").unwrap().to_string(),
                    name: s.get("name").unwrap().to_string(),
                    directory: s.get("directory").unwrap().to_string(),
                    constraint: s.get("constraint").is_none().then(|| String::from("")),
                    color: s.get("color").is_none().then(|| String::from(""))
                });
            }
        }
    }

    storages
}

#[tauri::command]
pub fn create_storage(s: StorageArgs) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let dir = Base::init_path().get_base_directory();

    // Hashing and take this as filename
    let hasher = HashHandler::encrypt(&s.name);

    handler
        .create_file(&dir, &hasher)
        .compose(&Storage::serialize(NewStorage {
            name: &s.name,
            directory: &s.directory,
            constraint: s.constraint,
            color: s.color
        }))
}

#[tauri::command]
pub fn update_storage(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path().get_base_directory();
    let file = format!("{}/{}", &dir, &filename);

    let mut content = handler.retrieve(&file).read_content();

    handler.update(
        &mut content,
        TOMLUpdateArgs {
            key: "storage",
            from: MappedField {
                field: target.field,
                value: target.value,
            },
        },
    )
}
