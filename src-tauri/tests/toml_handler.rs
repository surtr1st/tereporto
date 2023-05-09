#[cfg(test)]
mod toml_handler_on_action {
    use std::{thread, time::Duration};
    use tereporto::{hash_handler::HashHandler, storage, teleport, toml_handler::TOMLHandler};

    const FILENAME: &str = "links.toml";
    const TELEPORT_NAME: &str = "Teleport Folder";
    const STORAGE_NAME: &str = "Storage Folder";
    #[test]
    fn write_teleport_content() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::new(FILENAME);
        let teleport_index = HashHandler::encrypt(&TELEPORT_NAME);
        let storage_index = HashHandler::encrypt(&STORAGE_NAME);
        let teleport = teleport::Teleport {
            index: teleport_index,
            name: TELEPORT_NAME.to_string(),
            directories: vec!["/a/b/c".to_string()],
            to: storage_index,
        };

        handler.compose(&teleport.serialize());
    }

    #[test]
    fn write_storage_content() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::new(FILENAME);
        let teleport_index = HashHandler::encrypt(&TELEPORT_NAME);
        let storage_index = HashHandler::encrypt(&STORAGE_NAME);
        let store = storage::Storage {
            index: storage_index,
            name: STORAGE_NAME.to_string(),
            directory: "/x/y/z".to_string(),
            constraint: teleport_index
        };

        handler.compose(&store.serialize());
    }
}

#[cfg(test)]
mod toml_handler_on_validation {
    use std::{env, fs::File, path::Path, thread, time::Duration};
    use tereporto::toml_handler::TOMLHandler;

    const BASE_DIR: &str = "HOME";
    const FOLDER: &str = ".tereporto";
    const FILENAME: &str = "links.toml";
    const TELEPORT_NAME: &str = "Teleport Folder";
    const STORAGE_NAME: &str = "Storage Folder";

    #[test]
    fn default_init() {
        let reader = TOMLHandler::new(FILENAME);
        let default_dir = format!("{}/{}", env::var(BASE_DIR).unwrap(), &FOLDER);
        assert_eq!(reader.directory, default_dir);
        assert_eq!(reader.filename, FILENAME);
    }

    #[test]
    fn folder_existed() {
        thread::sleep(Duration::from_millis(500));
        let dir_folder = format!("{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER);
        let path = Path::new(&dir_folder).exists();
        assert_eq!(path, true);
    }

    #[test]
    fn file_existed() {
        thread::sleep(Duration::from_millis(500));
        let dir_file = format!("{}/{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER, &FILENAME);
        let file = File::open(dir_file).is_ok();
        assert_eq!(file, true);
    }

    #[test]
    fn teleport_block_existed() {
        thread::sleep(Duration::from_millis(500));
        let mut reader = TOMLHandler::new(FILENAME);
        let data = reader.read_from_file();
        let teleport = &data["teleports"];
        let teleport_name = teleport["name"].as_str().unwrap();
        assert_eq!(teleport_name, TELEPORT_NAME.to_string());
    }

    #[test]
    fn storage_block_existed() {
        thread::sleep(Duration::from_millis(500));
        let mut reader = TOMLHandler::new(FILENAME);
        let data = reader.read_from_file();
        let storage = &data["storage"];
        let storage_name = storage["name"].as_str().unwrap();
        assert_eq!(storage_name, STORAGE_NAME.to_string());
    }

    #[test]
    fn constrainted_with_storage() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::new(FILENAME);
        let data = handler.read_from_file();
        let teleport = &data["teleports"];
        let storage = &data["storage"];
        let teleport_to = teleport["to"].as_str().unwrap();
        let storage_index = storage["index"].as_str().unwrap();
        assert_eq!(teleport_to, storage_index);
    }

    #[test]
    fn constrainted_with_teleport() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::new(FILENAME);
        let data = handler.read_from_file();
        let teleport = &data["teleports"];
        let storage = &data["storage"];
        let teleport_index = teleport["index"].as_str().unwrap();
        let storage_constraint = storage["constraint"].as_str().unwrap();
        assert_eq!(teleport_index, storage_constraint);
    }
}

#[cfg(test)]
mod toml_handler_on_modification {
    use std::{thread, time::Duration};
    use tereporto::toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs};
    const FILENAME: &str = "links.toml";

    #[test]
    fn update_teleport_directories() {
        thread::sleep(Duration::from_secs(1));
        env_logger::init();
        let handler = TOMLHandler::new(FILENAME);
        handler.clone().update(TOMLUpdateArgs {
            key: "teleports",
            from: MappedField {
                field: "directories",
                value: "/a/du/dark/wa/123",
            },
        });
        let data = handler.clone().read_from_file();
        let dirs = data["teleports"]["directories"].as_array().unwrap();
        assert!(dirs.len() > 1);
    }
}
