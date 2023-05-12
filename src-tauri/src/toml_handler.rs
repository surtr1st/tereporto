use std::{fs, path::Path};

#[derive(Debug, Default, Clone)]
pub struct TOMLHandler {
    pub filename: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct MappedField<'mf> {
    pub field: &'mf str,
    pub value: &'mf str,
}

#[derive(Debug, Default, Clone)]
pub struct TOMLUpdateArgs<'tua> {
    pub key: &'tua str,
    pub to: MappedField<'tua>,
}

impl TOMLHandler {
    pub fn create_file(&mut self, dir: &str, filename: &str) -> &mut TOMLHandler {
        let file = format!("{}/{}.toml", dir, filename);

        // Create file if it doesn't exist
        if !Path::new(&file).is_file() {
            fs::File::create(&file).unwrap();
        }

        self.filename = file;
        self
    }

    pub fn retrieve(&mut self, filename: &str) -> &mut TOMLHandler {
        self.filename = filename.to_string();
        self
    }

    pub fn read_content(&self) -> toml::Value {
        let content = fs::read_to_string(&self.filename).unwrap();
        // Parse the TOML string into `toml::Value`
        content
            .parse::<toml::Value>()
            .expect("should be parsed TOML string into toml::Value")
    }

    pub fn compose(&mut self, target: &str) -> Result<String, String> {
        let mut table = self.read_content();

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
        match fs::write(&self.filename, updated_content) {
            Ok(f) => Ok(format!("Composed and updated file: {:#?}!", f)),
            Err(_) => Err(String::from("Could not update or compose!")),
        }
    }

    pub fn update(
        &mut self,
        content: &mut toml::Value,
        target: TOMLUpdateArgs,
    ) -> Result<String, String> {
        // Update the array by specifying its key
        if let Some(table) = content
            .get_mut(target.key)
            .and_then(|value| value.as_table_mut())
        {
            if let Some(dirs) = table
                .get_mut(target.to.field)
                .and_then(|value| value.as_array_mut())
            {
                dirs.push(toml::Value::String(target.to.value.to_string()));
            }
        }

        // Serialize the updated TOML back to a string
        let data =
            toml::to_string_pretty(&content).expect("should be serialized the data back to string");

        // Write the updated TOML back to the file
        self.compose(&data)
    }
}
