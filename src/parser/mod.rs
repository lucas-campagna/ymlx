mod constants;
mod apply;
mod utils;
mod runtime;
mod component;

use std::ops::Deref;
use runtime::Runtime;
use rust_yaml::{Error, Value, Yaml};
use component::Component;
pub struct Parser(Value);

impl Deref for Parser {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    pub fn call(&self, name: &str, props: Value) -> Result<Component, Error> {
        let mut runtime = Runtime::new(self.deref());
        let value = runtime.call(name, props)?;
        Ok(Component::new(value))
    }
    pub fn to_yaml(&self) -> Result<String, Error> {
        Yaml::new().dump_str(self)
    }
    pub fn to_json(&self) -> String {
        self.deref().to_string()
    }
    pub fn to_component(&self) -> Component {
        Component::new(self.to_value())
    }
    pub fn to_value(&self) -> Value {
        self.deref().clone()
    }
}