
use toml_edit::Document;

#[derive(Debug, Default, serde::Serialize)]
pub struct Storage {
    pub name: String,
    pub directories: Vec<StorageDirectory>
}

#[derive(Debug, Default, serde::Serialize)]
pub struct StorageDirectory {
    pub dir: String,
    pub primary: bool
}

#[derive(Debug, Default, serde::Serialize)]
pub struct StorageBox {
    pub storage: Storage
}

impl Storage {
    pub fn create(self) -> String {
        let template = format!(r#"
            storage = {{ name = {}, directories = {:?} }}
        "#, self.name, self.directories);
        let doc = template.parse::<Document>().expect("invalid document");
        doc.to_string()
    }
}
