use crate::base::{DirectoryControl, Base};
use crate::hash_handler::HashHandler;
use crate::teleport::{TeleportArgs, Teleport, NewTeleport};
use crate::toml_handler::{TOMLHandler, TOMLUpdateArgs, MappedField};


#[tauri::command]
pub fn create_teleport(t: TeleportArgs) -> Result<String, String> {
    let mut handler = TOMLHandler::default();

    let base = Base::init_path();
    Base::create_folder(&t.directories[0], &t.name);

    // Hashing and take this as filename
    let hasher = HashHandler::encrypt(&t.name);

    handler
        .create_file(&base.get_base_directory(), &hasher)
        .compose(&Teleport::serialize(NewTeleport {
            name: &t.name,
            directories: &t.directories,
            to: t.to
        }))
}

#[tauri::command]
pub fn update_teleport(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = Base::init_path().get_base_directory();
    let file = format!("{}/{}", &dir, &filename);

    let mut content = handler
        .retrieve(&file)
        .read_content();

    handler
        .update(&mut content, TOMLUpdateArgs { 
            key: "teleports", 
            from: MappedField {
                field: target.field,
                value: target.value
            }
        })
}
