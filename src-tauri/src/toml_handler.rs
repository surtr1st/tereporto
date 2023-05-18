use crate::helpers::remove_quotes;
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
            let field = table.get_mut(target.to.field);
            if field.is_some() {
                if let Some(f) = field {
                    match f {
                        toml::Value::String(_) => {
                            *f = toml::Value::from(remove_quotes(target.to.value));
                        }
                        toml::Value::Boolean(_) => {
                            *f = toml::Value::from(target.to.value.parse::<bool>().unwrap());
                        }
                        toml::Value::Integer(_) => {
                            *f = toml::Value::from(target.to.value.parse::<i64>().unwrap());
                        }
                        _ => {}
                    }
                }
            } else {
                table.insert(
                    target.to.field.to_owned(),
                    toml::Value::from(target.to.value),
                );
            }

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

    pub fn remove_field(
        &mut self,
        content: &mut toml::Value,
        key: &str,
        field: &str,
    ) -> Result<String, String> {
        // Check if the key exists
        if let Some(table) = content.get_mut(key).and_then(toml::Value::as_table_mut) {
            // Remove the field if it exists
            table.remove(field);
        }

        // Serialize the modified TOML back to a string
        let updated =
            toml::to_string_pretty(&content).expect("should be serialized the data back to string");
        self.compose(&updated)
    }

    pub fn remove(&mut self, filename: &str) -> Result<String, String> {
        match fs::remove_file(filename) {
            Ok(_) => Ok(format!("Removed: {}", filename)),
            Err(_) => panic!("Cannot remove: {}", filename),
        }
    }

    pub fn get_content<'c>(
        &self,
        content: &'c toml::Value,
        key: &'c str,
    ) -> Result<&'c toml::map::Map<String, toml::Value>, String> {
        if let Some(section) = content.get(key) {
            if let Some(field) = section.as_table() {
                return Ok(field);
            }
        }
        Err(String::from("No content"))
    }
}
