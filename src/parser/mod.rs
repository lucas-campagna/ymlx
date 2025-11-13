use std::collections::HashMap;
use rust_yaml::Yaml;
use types::*;

mod types;

pub fn parse(code: &str) -> Result<Document, Box<dyn std::error::Error>> {
    let yaml = Yaml::new();
    let parsed: HashMap<String, Properties> = yaml.load_str(code)?.as_mapping().unwrap();

    let mut templates = HashMap::new();
    let mut components = HashMap::new();
    let mut entry_points = HashMap::new();

    for (key, value) in parsed {
        if key.starts_with("$(") && key.ends_with(")") {
            let selector = &key[2..key.len()-1];
        }
    }

    Ok(Document {
        templates,
        components,
        entry_points,
    })
    // Component {
    //     name: "".to_string(),
    //     body: ComponentBody::String("")
    // }
}