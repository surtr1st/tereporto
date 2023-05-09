use crate::base::{DirectoryControl, Base};
use crate::hash_handler::HashHandler;
use crate::storage::{StorageArgs, Storage, NewStorage};
use crate::toml_handler::{TOMLHandler, TOMLUpdateArgs, MappedField};


#[tauri::command]
pub fn create_storage(s: StorageArgs) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let base = Base::init_path();
    Base::create_folder(&s.directory, &s.name);

    // Hashing and take this as filename
    let hasher = HashHandler::encrypt(&s.name);

    handler
        .create_file(&base.get_base_directory(), &hasher)
        .compose(&Storage::serialize(NewStorage {
            name: &s.name,
            directory: &s.directory,
            constraint: s.constraint
        }))
}

#[tauri::command]
pub fn update_storage(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path().get_base_directory();
    let file = format!("{}/{}", &dir, &filename);

    let mut content = handler
        .retrieve(&file)
        .read_content();

    handler
        .update(&mut content, TOMLUpdateArgs { 
            key: "storage", 
            from: MappedField {
                field: target.field,
                value: target.value
            }
        })
}
