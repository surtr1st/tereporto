#[cfg(test)]
mod toml_handler_on_action {
    use std::{thread, time::Duration};
    use tereporto::{
        base::{Base, DirectoryControl},
        hash_handler::HashHandler,
        storage::{self, Storage},
        teleport::{self, Teleport},
        toml_handler::TOMLHandler,
    };

    const FILENAME: &str = "links.toml";
    const TELEPORT_NAME: &str = "Teleport Folder";
    const STORAGE_NAME: &str = "Storage Folder";

    #[test]
    fn create_toml_file() {
        thread::sleep(Duration::from_millis(500));
        let dir = Base::init_path().get_base_directory();
        let mut handler = TOMLHandler::default();
        handler.create_file(&dir, FILENAME);
    }

    #[test]
    fn write_teleport_content() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::default();
        let storage_index = HashHandler::encrypt(&STORAGE_NAME);

        let teleport = teleport::NewTeleport {
            name: TELEPORT_NAME,
            directories: &vec!["/a/b/c".to_string()],
            to: Some(storage_index),
        };

        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        handler
            .retrieve(&file)
            .compose(&Teleport::serialize(teleport))
            .unwrap();
    }

    #[test]
    fn write_storage_content() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::default();
        let teleport_index = HashHandler::encrypt(&TELEPORT_NAME);

        let store = storage::NewStorage {
            name: STORAGE_NAME,
            directory: "/x/y/z",
            constraint: Some(teleport_index),
        };

        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        handler
            .retrieve(&file)
            .compose(&Storage::serialize(store))
            .unwrap();
    }
}

#[cfg(test)]
mod toml_handler_on_validation {
    use std::{env, fs::File, path::Path, thread, time::Duration};
    use tereporto::{
        base::{Base, DirectoryControl},
        toml_handler::TOMLHandler,
    };

    const BASE_DIR: &str = "HOME";
    const FOLDER: &str = ".tereporto";
    const FILENAME: &str = "links.toml";
    const TELEPORT_NAME: &str = "Teleport Folder";
    const STORAGE_NAME: &str = "Storage Folder";

    #[test]
    fn folder_existed() {
        thread::sleep(Duration::from_millis(500));
        let dir_folder = format!("{}/{}", env::var(BASE_DIR).unwrap(), FOLDER);
        let path = Path::new(&dir_folder).exists();
        assert_eq!(path, true);
    }

    #[test]
    fn file_existed() {
        thread::sleep(Duration::from_millis(500));
        let dir = Base::init_path().get_base_directory();
        let dir_file = format!("{}/{}", &dir, FILENAME);
        let file = File::open(dir_file).is_ok();
        assert_eq!(file, true);
    }

    #[test]
    fn teleport_block_existed() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        let data = handler.retrieve(&file).read_content();

        let teleport = &data["teleports"];
        let teleport_name = teleport["name"].as_str().unwrap();
        assert_eq!(teleport_name, TELEPORT_NAME.to_string());
    }

    #[test]
    fn storage_block_existed() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        let data = handler.retrieve(&file).read_content();

        let storage = &data["storage"];
        let storage_name = storage["name"].as_str().unwrap();
        assert_eq!(storage_name, STORAGE_NAME.to_string());
    }

    #[test]
    fn constrainted_with_storage() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        let data = handler.retrieve(&file).read_content();

        let teleport = &data["teleports"];
        let storage = &data["storage"];
        let teleport_to = teleport["to"].as_str().unwrap();
        let storage_index = storage["index"].as_str().unwrap();
        assert_eq!(teleport_to, storage_index);
    }

    #[test]
    fn constrainted_with_teleport() {
        thread::sleep(Duration::from_millis(500));
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        let data = handler.retrieve(&file).read_content();

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
    use tereporto::{
        base::{Base, DirectoryControl},
        toml_handler::{MappedField, TOMLHandler, TOMLUpdateArgs},
    };

    const FILENAME: &str = "links.toml";

    #[test]
    fn update_teleport_directories() {
        thread::sleep(Duration::from_secs(1));
        env_logger::init();
        let mut handler = TOMLHandler::default();
        let dir = Base::init_path().get_base_directory();
        let file = format!("{}/{}", &dir, FILENAME);
        let mut data = handler.retrieve(&file).read_content();

        handler
            .update(
                &mut data,
                TOMLUpdateArgs {
                    key: "teleports",
                    from: MappedField {
                        field: "directories",
                        value: "/a/du/dark/wa/123",
                    },
                },
            )
            .unwrap();
        let dirs = data["teleports"]["directories"].as_array().unwrap();
        assert!(dirs.len() > 1);
    }
}
