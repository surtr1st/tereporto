use crate::base::get_base_directory;
use crate::teleport::{TeleportArgs, Teleport, NewTeleport};
use crate::toml_handler::{TOMLHandler, TOMLUpdateArgs, MappedField};


#[tauri::command]
pub fn create_teleport(t: TeleportArgs) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let dir = &get_base_directory();

    handler
        .create_file(dir, &t.name)
        .compose(&Teleport::serialize(NewTeleport {
            name: &t.name,
            directories: &t.directories,
            to: t.to
        }))
}

#[tauri::command]
pub fn update_teleport(filename: String, target: MappedField) -> Result<String, String> {
    let mut handler = TOMLHandler::default();
    let file = format!("{}/{}", &get_base_directory(), &filename);

    let mut content = handler
        .retrieve(&file)
        .read_content();

    handler
        .update(&mut content, TOMLUpdateArgs { 
            key: "teleport", 
            from: MappedField {
                field: target.field,
                value: target.value
            }
        })
}

