use std::collections::HashMap;

use crate::{
    base::{Base, DirectoryControl},
    constants::SETTINGS_FILE,
    settings::Settings,
    toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs},
};

#[tauri::command]
pub fn load_settings() -> Result<Settings, String> {
    Ok(Settings::load())
}

#[tauri::command]
pub fn save_settings(options: HashMap<String, String>) {
    let mut handler = TOMLHandler::default();
    let base = Base::init_path().get_base_directory();
    let config = format!("{}/{}.toml", base, SETTINGS_FILE);
    let mut content = handler.retrieve(&config).read_content();

    options.iter().for_each(|(key, value)| {
        handler
            .update(
                &mut content,
                TOMLUpdateArgs {
                    key: SETTINGS_FILE,
                    to: MappedField { field: key, value },
                },
            )
            .unwrap();
    });
}
