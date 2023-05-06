use std::{env, fs, path::Path};

const BASE_FILENAME: &str = "links.toml";

#[derive(Debug, Default, Clone)]
pub struct TOMLHandler {
    pub directory: String,
    pub filename: String,
}

impl TOMLHandler {
    pub fn new() -> Self {
        let root = env::var("HOME").unwrap();
        let store_folder = ".tereporto";
        let default_path = format!("{}/{}", root, store_folder);
        let default_file = format!("{}/{}", default_path, BASE_FILENAME);

        // Create directory if it doesn't exist
        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
        }

        // Create file if it doesn't exist
        if !Path::new(&default_file).is_file() {
            fs::File::create(&default_file).unwrap();
        }
        
        TOMLHandler {
            directory: default_path,
            filename: BASE_FILENAME.to_string(),
        }
    }

    pub fn read_from_file(self) -> toml::Value {
        let file = format!("{}/{}", self.directory, self.filename);
        let content = fs::read_to_string(file).unwrap();
        content.parse::<toml::Value>().unwrap()
    }

    pub fn compose(self, target: String) {
        // Read the existing TOML file into a string
        let file = format!("{}/{}", self.directory, self.filename);
        let existing_content = fs::read_to_string(&file).unwrap();

        // Parse the TOML string into a `toml::Value`
        let mut toml_value = existing_content.parse::<toml::Value>().unwrap();

        // Modify the parsed TOML value by adding or updating the desired fields
        let additional_content = target.parse::<toml::Value>().unwrap();
        if let toml::Value::Table(existing_table) = &mut toml_value {
            if let toml::Value::Table(additional_table) = additional_content {
                existing_table.extend(additional_table);
            }
        }

        // Serialize the modified TOML value back into a TOML string
        let updated_content = toml::to_string_pretty(&toml_value).unwrap();

        // Write the TOML string back to the original file (overwrite the file)
        fs::write(file, updated_content).unwrap();
    }

    pub fn update(self, key: String, field: String, value: String) {
        // Read the TOML file
        let file = format!("{}/{}", self.directory, self.filename);
        let mut data = fs::read_to_string(&file).unwrap();

        // Parse the TOML into a Value
        let mut content: toml::Value = toml::from_str(&data)
            .expect("Failed to parse TOML");

        // Update the array by specifying its key
        if let Some(table) = content.get_mut(&key).and_then(|value| value.as_table_mut()) {
            if let Some(dirs) = table.get_mut(&field).and_then(|value| value.as_array_mut()) {
                dirs.push(toml::Value::String(value));
            }
        }

        // Serialize the updated TOML back to a string
        data = toml::to_string_pretty(&content).unwrap();

        // Write the updated TOML back to the file
        self.compose(data);
    }
}
