#![allow(dead_code, unused)]
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::{
    base::{Base, DirectoryControl},
    constants::TELEPORT_ARCHIVE_FOLDER,
    hash_handler::HashHandler,
    toml_handler::TOMLHandler,
};
use regex::Regex;

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
    println!("{}", dir);
    match open::that(dir) {
        Ok(status) => Ok(()),
        Err(_) => Err(String::from("Cannot open selected directory!")),
    }
}

pub fn remove_quotes(target: &str) -> String {
    let single_quote = Regex::new("\'").unwrap();
    let quotes = Regex::new("\"").unwrap();
    let remove_single = format!("{}", single_quote.replace_all(target, ""));
    format!("{}", quotes.replace_all(&remove_single, ""))
}

pub fn retrieve_directories(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>()
}

pub fn retrieve_directory_content(dir: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() || path.is_dir())
        .collect::<Vec<_>>()
}

pub fn retrieve_directory_files(dir: &str) -> Vec<DirEntry> {
    fs::read_dir(dir)
        .expect("should read the directory specified!")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .collect::<Vec<_>>()
}

pub fn convert_to_linux_path(path: &str) -> String {
    let path_str = path.to_string();
    path_str.replace("\\", "/")
}

pub fn is_windows_path(path: &str) -> bool {
    path.contains("\\")
}
