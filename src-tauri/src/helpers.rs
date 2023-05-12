#![allow(dead_code, unused)]
use crate::{
    base::{Base, DirectoryControl},
    hash_handler::HashHandler,
    toml_handler::TOMLHandler,
};

pub const TELEPORT_ARCHIVE_FOLDER: &str = "teleports";
pub const STORAGE_ARCHIVE_FOLDER: &str = "storages";

pub struct ConnectionBetween<'cb> {
    pub teleport: &'cb str,
    pub storage: &'cb str,
}

pub fn has_connected(c: ConnectionBetween) -> bool {
    let mut handler = TOMLHandler::default();

    let filename_encrypted = (
        HashHandler::encrypt(c.teleport),
        HashHandler::encrypt(c.storage),
    );

    let first_file = format!(
        "{}/{}",
        Base::init_path()
            .get_recursive(TELEPORT_ARCHIVE_FOLDER)
            .get_base_directory(),
        filename_encrypted.0
    );
    let second_file = format!(
        "{}/{}",
        Base::init_path()
            .get_recursive(STORAGE_ARCHIVE_FOLDER)
            .get_base_directory(),
        filename_encrypted.1
    );

    let t = handler.retrieve(&first_file).read_content();
    let s = handler.retrieve(&second_file).read_content();

    let mut indexes = vec![];
    if let Some(section) = t.get("teleports") {
        if let Some(teleport) = section.as_table() {
            indexes.push(teleport.get("to").unwrap().to_string());
        }
    }

    if let Some(section) = s.get("storage") {
        if let Some(storage) = section.as_table() {
            indexes.push(storage.get("constraint").unwrap().to_string());
        }
    }

    indexes[0] == indexes[1]
}

#[tauri::command]
pub fn open_selected_directory(dir: &str) -> Result<(), String> {
    println!("{}", dir);
    match open::that(dir) {
        Ok(status) => Ok(()),
        Err(_) => Err(String::from("Cannot open selected directory!")) 
    }
}
