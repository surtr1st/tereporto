use crate::base::{Base, DirectoryControl};
use crate::hash_handler::HashHandler;
use crate::storage::{NewStorage, Storage, StorageArgs};
use crate::toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs};

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
