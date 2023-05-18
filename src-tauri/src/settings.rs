use crate::{
    base::{Base, DirectoryControl},
    constants::SETTINGS_FILE,
    helpers::remove_quotes,
    toml_handler::TOMLHandler,
};
use std::path::Path;

use tauri::{AppHandle, GlobalWindowEvent, RunEvent, WindowEvent};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub auto_scan: bool,
    pub preferred_lang: String,
    pub close_mode: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SettingsBox {
    pub settings: Settings,
}

impl Settings {
    pub fn load() -> Self {
        let base = Base::init_path().get_base_directory();
        let config_dir = format!("{}/{}.toml", base, SETTINGS_FILE);

        let mut handler = TOMLHandler::default();

        if !Path::new(&config_dir).is_file() {
            handler.create_file(&base, SETTINGS_FILE);
            let default = SettingsBox {
                settings: Settings {
                    auto_scan: false,
                    preferred_lang: String::from("en"),
                    close_mode: String::from("default"),
                },
            };
            let default_settings = toml::to_string_pretty(&default).unwrap();
            handler.compose(&default_settings).unwrap();
        }

        let content = handler.retrieve(&config_dir).read_content();

        let section = content.get("settings").unwrap();
        let option = section.as_table().unwrap();
        let auto_scan = option.get("auto_scan").unwrap().as_bool().unwrap();
        let preferred_lang = option.get("preferred_lang").unwrap().to_string();
        let close_mode = option.get("close_mode").unwrap().to_string();

        Settings {
            auto_scan,
            preferred_lang,
            close_mode,
        }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemSettings;

pub trait SystemSettingsTrait {
    fn prevent_frontend_on_close(event: GlobalWindowEvent);
    fn prevent_backend_on_close(app: &AppHandle, event: RunEvent);
}

impl SystemSettingsTrait for SystemSettings {
    fn prevent_frontend_on_close(event: GlobalWindowEvent) {
        let close_mode = remove_quotes(&Settings::load().close_mode);
        if close_mode == "minimized" {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        }
    }

    fn prevent_backend_on_close(_app: &AppHandle, event: RunEvent) {
        let close_mode = remove_quotes(&Settings::load().close_mode);
        if close_mode == "minimized" {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        }
    }
}
