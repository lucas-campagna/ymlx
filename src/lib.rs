mod context;
mod discoverable_key;
mod processing_context;
mod utils;
mod value;

use context::Context;
use regex::Regex;
use rust_yaml::Yaml;
use thiserror::Error;
pub use value::Value;

pub fn parse(code: &str) -> Result<Context, ParseError> {
    let re = Regex::new(r"\$(\w+)").unwrap();
    let parsed_code = re.replace_all(code, |caps: &regex::Captures| format!("${{{}}}", &caps[1]));
    let result = Yaml::new().load_str(&parsed_code)?;
    match result.into() {
        Value::Mapping(result) => Ok(Context::build(result)),
        _ => Err(ParseError::InvalidYamlBaseValue),
    }
}

#[derive(Error, Debug)]
pub enum MapValuesError {
    #[error("Yaml mapping keys should be strings only")]
    InvalidYamlMappingKeys,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Error while parsing the YAML: {0}")]
    YamlParsingError(#[from] rust_yaml::Error),
    #[error("Yaml file should be key-value pair on top level")]
    InvalidYamlBaseValue,
    #[error("{0}")]
    InvalidYamlMappingKeys(#[from] MapValuesError),
}

#[cfg(test)]
mod test {
    use super::*;
    use discoverable_key::DiscoverableKey;
    #[test]
    fn build_discoverable_component_names_test() {
        {
            let k = DiscoverableKey("from");
            assert!(k.matches("from!"));
            assert!(k.matches("From"));
            assert!(k.matches("yx-from"));
        }
        {
            let k = DiscoverableKey("MyDb");
            assert!(k.matches("MyDb!"));
            assert!(k.matches("MyDb"));
            assert!(k.matches("yx-MyDb"));
        }
    }
}
