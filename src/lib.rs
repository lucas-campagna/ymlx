mod context;
mod discoverable_key;
mod merge;
mod processing_context;
mod value;

use context::Context;
use serde_yaml_ng::from_str;
use thiserror::Error;
pub use value::Value;

pub fn parse(code: &str) -> Result<Context, ParseError> {
    let result: serde_yaml_ng::Value = from_str(&code)?;
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
    YamlParsingError(#[from] serde_yaml_ng::Error),
    #[error("Yaml file should be key-value pair on top level")]
    InvalidYamlBaseValue,
    #[error("{0}")]
    InvalidYamlMappingKeys(#[from] MapValuesError),
}
