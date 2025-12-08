use std::io::Read;
use rust_yaml::Yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = Yaml::new();

    // Parse YAML from a string
    let mut yaml_content = String::new();
    std::io::stdin().read_to_string(&mut yaml_content)?;

    let parsed = yaml.load_str(yaml_content.as_str())?;
    println!("Parsed: {:?}", parsed.to_string());

    // Dump back to YAML
    let output = yaml.dump_str(&parsed)?;
    println!("Output:\n{}", output);

    Ok(())
}
