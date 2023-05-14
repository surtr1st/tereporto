#![allow(dead_code, unused)]
use std::{fs, path::PathBuf};

use crate::{
    base::{Base, DirectoryControl},
    hash_handler::HashHandler,
    toml_handler::TOMLHandler,
};
use regex::Regex;

pub const TELEPORT_ARCHIVE_FOLDER: &str = "teleports";
pub const STORAGE_ARCHIVE_FOLDER: &str = "storages";

pub struct ConnectionBetween<'cb> {
    pub teleport_index: &'cb str,
    pub storage_index: &'cb str,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Constraint {
    pub current_connect: String,
    pub directory: String,
}

#[tauri::command]
pub fn open_selected_directory(dir: &str) -> Result<(), String> {
    match open::that(dir) {
        Ok(status) => Ok(()),
        Err(_) => Err(String::from("Cannot open selected directory!")),
    }
}

pub fn has_connected(c: ConnectionBetween) -> bool {
    let mut handler = TOMLHandler::default();

    let teleport = format!(
        "{}/{}",
        Base::init_path()
            .get_recursive(TELEPORT_ARCHIVE_FOLDER)
            .get_base_directory(),
        c.teleport_index
    );
    let storage = format!(
        "{}/{}",
        Base::init_path()
            .get_recursive(STORAGE_ARCHIVE_FOLDER)
            .get_base_directory(),
        c.storage_index
    );

    let t = handler.retrieve(&teleport).read_content();
    let s = handler.retrieve(&storage).read_content();

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

pub fn remove_quotes(target: &str) -> String {
    let regex = Regex::new("\"").unwrap();
    format!("{}", regex.replace_all(target, ""))
}

pub fn retrieve_directory_content(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() || path.is_dir())
        .collect::<Vec<_>>()
}
