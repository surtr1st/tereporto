#[cfg(test)]
mod toml_reader_tests {
    use tereporto::{toml_handler::TOMLHandler, storage, teleport, hash_handler::HashHandler};
    use std::{env, fs::File, path::Path};
    const BASE_DIR: &str = "HOME";
    const FOLDER: &str = ".tereporto";
    const FILENAME: &str = "links.toml";

    #[test]
    fn default_init() {
        let reader = TOMLHandler::new();
        let default_dir = format!("{}/{}", env::var(BASE_DIR).unwrap(), &FOLDER);
        assert_eq!(reader.directory, default_dir);
        assert_eq!(reader.filename, FILENAME);
    }

    #[test]
    fn folder_existed() {
        let dir_folder = format!("{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER);
        let path = Path::new(&dir_folder).exists();
        assert_eq!(path, true);
    }

    #[test]
    fn file_existed() {
        let dir_file = format!("{}/{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER, &FILENAME);
        let file = File::open(dir_file).is_ok();
        assert_eq!(file, true);
    }

    #[test]
    fn read_file_content() {
        let reader = TOMLHandler::new();
        let data = reader.read_from_file();
        // [<hash>]
        let link = &data["Twitter"];
        // teleports = { name: <hash>, directories: [...] }
        let teleports = link["teleports"]["name"].as_str().unwrap();
        // storage = { name: <hash>, directories: [{ dir: ..., primary: true|false }] }
        let storage = link["storage"]["name"].as_str().unwrap();
        assert_eq!(teleports, "adudarkwa");
        assert_eq!(storage, "123456");
    }


    // #[test]
    // fn create_key() {
    //     let handler = TOMLHandler::new();
    //     let key = HashHandler::encrypt("Test");
    //     let toml_key = toml::to_string_pretty(&key).unwrap();
    //     handler.create_unique_key(key.clone());
    //     assert_eq!(key, toml_key);
    // }

    #[test]
    fn write_toml_content() {
        let handler = TOMLHandler::new();
        let store = storage::Storage {
            name: "Storage Folder".to_string(),
            directories: vec![
                storage::StorageDirectory {
                    dir: "/x/y/z".to_string(),
                    primary: true
                } 
            ]
        };
        let tele = teleport::Teleport {
            name: "Teleport Folder".to_string(),
            directories: vec!["/a/b/c".to_string()]
        };

        let key = HashHandler::encrypt("Test");
        // handler.clone().compose(tele.create());
        handler.clone().compose(store.create());
    }
}
