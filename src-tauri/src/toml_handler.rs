use std::{env, fs, path::Path};

#[derive(Debug, Default, Clone)]
pub struct TOMLHandler {
    pub directory: String,
    pub filename: String,
}

#[derive(Debug, Default, Clone)]
pub struct MappedField<'mf> {
    pub field: &'mf str,
    pub value: &'mf str,
}

#[derive(Debug, Default, Clone)]
pub struct TOMLUpdateArgs<'tua> {
    pub key: &'tua str,
    pub from: MappedField<'tua>,
}

impl TOMLHandler {
    pub fn new(filename: &str) -> Self {
        let root = env::var("HOME").unwrap();
        let store_folder = ".tereporto";
        let default_path = format!("{}/{}", root, store_folder);
        let default_file = format!("{}/{}", default_path, filename);

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
            filename: filename.to_string(),
        }
    }

    pub fn change_default_directory(&mut self, dir: &str) {
        self.directory = dir.to_string();
    }

    pub fn read_from_file(self) -> toml::Value {
        let file = format!("{}/{}", self.directory, self.filename);
        let content = fs::read_to_string(file).unwrap();
        // Parse the TOML string into `toml::Value`
        content
            .parse::<toml::Value>()
            .expect("should be parsed TOML string into toml::Value")
    }

    pub fn compose(self, target: &str) {
        let file = format!("{}/{}", self.directory, self.filename);

        // Read the TOML file
        let current_content =
            fs::read_to_string(&file).unwrap_or_else(|_| panic!("should have read file: {}", self.filename));

        // Parse the TOML string into a `toml::Value`
        let mut table = current_content
            .parse::<toml::Value>()
            .expect("should be parsed TOML string into toml::Value");

        // Modify the parsed TOML value by adding or updating the desired fields
        let additional_content = target.parse::<toml::Value>().unwrap();
        if let toml::Value::Table(existing_table) = &mut table {
            if let toml::Value::Table(additional_table) = additional_content {
                existing_table.extend(additional_table);
            }
        }

        // Serialize the modified TOML value back into a TOML string
        let updated_content = toml::to_string_pretty(&table).unwrap();

        // Write the TOML string back to the original file (overwrite the file)
        fs::write(file, updated_content).unwrap();
    }

    pub fn update(self, target: TOMLUpdateArgs) {
        let file = format!("{}/{}", self.directory, self.filename);

        // Read the TOML file
        let mut data =
            fs::read_to_string(file).unwrap_or_else(|_| panic!("should have read file: {}", self.filename));

        // Parse the TOML into a `toml::Value`
        let mut content: toml::Value =
            toml::from_str(&data).expect("should be parsed TOML string into toml::Value");

        // Update the array by specifying its key
        if let Some(table) = content
            .get_mut(target.key)
            .and_then(|value| value.as_table_mut())
        {
            if let Some(dirs) = table
                .get_mut(target.from.field)
                .and_then(|value| value.as_array_mut())
            {
                dirs.push(toml::Value::String(target.from.value.to_string()));
            }
        }

        // Serialize the updated TOML back to a string
        data =
            toml::to_string_pretty(&content).expect("should be serialized the data back to string");

        // Write the updated TOML back to the file
        self.compose(&data);
    }
}
