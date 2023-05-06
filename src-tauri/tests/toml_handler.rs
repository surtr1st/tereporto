#[cfg(test)]
mod toml_handler_tests {
    use tokio;
    use tereporto::{toml_handler::TOMLHandler, storage, teleport, hash_handler::HashHandler};
    use std::{env, fs::File, path::Path};
    use env_logger;

    const BASE_DIR: &str = "HOME";
    const FOLDER: &str = ".tereporto";
    const FILENAME: &str = "links.toml";
    const TELEPORT_NAME: &str = "Teleport Folder";
    const STORAGE_NAME: &str = "Storage Folder"; 

    #[tokio::test]
    async fn default_init() {
        let reader = TOMLHandler::new();
        let default_dir = format!("{}/{}", env::var(BASE_DIR).unwrap(), &FOLDER);
        assert_eq!(reader.directory, default_dir);
        assert_eq!(reader.filename, FILENAME);
    }

    #[tokio::test]
    async fn folder_existed() {
        let dir_folder = format!("{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER);
        let path = Path::new(&dir_folder).exists();
        assert_eq!(path, true);
    }

    #[tokio::test]
    async fn file_existed() {
        let dir_file = format!("{}/{}/{}", env::var(&BASE_DIR).unwrap(), &FOLDER, &FILENAME);
        let file = File::open(dir_file).is_ok();
        assert_eq!(file, true);
    } 

    #[tokio::test]
    async fn write_teleport_content() {
        let handler = TOMLHandler::new();
        let teleport_index = HashHandler::encrypt(&TELEPORT_NAME);
        let storage_index = HashHandler::encrypt(&STORAGE_NAME);
        let tele = teleport::Teleport {
            index: &teleport_index,
            name: TELEPORT_NAME.to_string(),
            directories: vec!["/a/b/c".to_string()],
            to: &storage_index
        };

        handler.compose(tele.create());
    }

    #[tokio::test]
    async fn write_storage_content() {
        let handler = TOMLHandler::new();
        let teleport_index = HashHandler::encrypt(&TELEPORT_NAME);
        let storage_index = HashHandler::encrypt(&STORAGE_NAME);
        let store = storage::Storage {
            index: &storage_index,
            name: STORAGE_NAME.to_string(),
            directory: "/x/y/z".to_string(),
            bind: &teleport_index,
            primary: true
        };
        
        handler.compose(store.create());
    }

    #[tokio::test]
    async fn teleport_block_existed() {
        let reader = TOMLHandler::new();
        let data = reader.read_from_file();
        let teleport = &data["teleports"];
        let teleport_name = teleport["name"].as_str().unwrap();
        assert_eq!(teleport_name, TELEPORT_NAME.to_string());
    }

    #[tokio::test]
    async fn storage_block_existed() {
        let reader = TOMLHandler::new();
        let data = reader.read_from_file();
        let storage = &data["storage"];
        let storage_name = storage["name"].as_str().unwrap();
        assert_eq!(storage_name, STORAGE_NAME.to_string());
    }

    #[tokio::test]
    async fn is_link_with_storage() {
        let handler = TOMLHandler::new();
        let data = handler.read_from_file();
        let teleport = &data["teleports"];
        let storage = &data["storage"];
        let teleport_to = teleport["to"].as_str().unwrap();
        let storage_index = storage["index"].as_str().unwrap();
        assert_eq!(teleport_to, storage_index);
    }

    #[tokio::test]
    async fn is_link_with_teleport() {
        let handler = TOMLHandler::new();
        let data = handler.read_from_file();
        let teleport = &data["teleports"];
        let storage = &data["storage"];
        let teleport_index = teleport["index"].as_str().unwrap();
        let storage_bind = storage["bind"].as_str().unwrap();
        assert_eq!(teleport_index, storage_bind);
    }

    #[tokio::test]
    async fn update_teleport_directories() {
        env_logger::init();
        let handler = TOMLHandler::new();
        handler.clone().update(
            "teleports".to_string(), 
            "directories".to_string(), 
            "/a/du/dark/wa".to_string()
        );
        let teleport = handler.clone().read_from_file();
        let dirs = teleport["teleports"]["directories"].as_array().unwrap();
        assert!(dirs.len() > 1);
    }
}
