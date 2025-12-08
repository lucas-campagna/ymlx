mod apply;
mod utils;
mod runtime;
mod component;

use runtime::Runtime;
use rust_yaml::{Error, Value};

use component::Component;

pub struct Parser(Value);

impl Parser {
    pub fn load(file: &str) -> Result<Parser, Error> {
        let input = std::fs::read_to_string(file)?;
        Ok(Parser::parse(&input)?)
    }
    pub fn parse(input: &str) -> Result<Parser, Error> {
        let value = rust_yaml::Yaml::new().load_str(input)?;
        match value {
            Value::Mapping(value) => Ok(Parser(Value::Mapping(value))),
            _ => Err(Error::emission("Root YAML is not a mapping")),
        }
    }
    pub fn call(&self, name: &str, props: &Value) -> Result<Component, Error> {
        let runtime = Runtime::new(&self.0);
        let value = runtime.call(name, props)?;
        Ok(Component::new(value))
    }
}

