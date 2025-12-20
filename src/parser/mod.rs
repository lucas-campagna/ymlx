mod constants;
mod apply;
mod utils;
mod runtime;
mod component;

use indexmap::IndexMap;
use runtime::Runtime;
use rust_yaml::{Error, Value, Yaml};
use component::Component;
use apply::clear_props;

pub struct Parser {
    components: Value,
    functions: IndexMap<String, fn(Value) -> Value>,
}

impl Parser {
    pub fn load(file: &str) -> Result<Parser, Error> {
        let input = std::fs::read_to_string(file)?;
        Ok(Parser::parse(&input)?)
    }

    pub fn from(components: Value, functions: IndexMap<String, fn(Value) -> Value>) -> Parser {
        Parser { components, functions }
    }

    pub fn from_components(components: Value) -> Parser {
        Parser::from(components, IndexMap::new())
    }
    
    pub fn parse(input: &str) -> Result<Parser, Error> {
        let value = rust_yaml::Yaml::new().load_str(input)?;
        match value {
            Value::Mapping(value) => Ok(Parser::from_components(Value::Mapping(value))),
            _ => Err(Error::emission("Root YAML is not a mapping")),
        }
    }

    pub fn add_function(&mut self, name: &str, function: fn(Value) -> Value) {
        self.functions.insert(name.into(), function);
    }

    pub fn add_functions(&mut self, functions: IndexMap<String, fn(Value) -> Value>) {
        self.functions.extend(functions);
    }

    pub fn call(&self, name: &str, props: Value) -> Result<Component, Error> {
        let mut runtime = Runtime::build(&self.components, &self.functions);
        let mut value = runtime.call(name, props)?;
        clear_props(&mut value);
        Ok(Component::new(value))
    }

    pub fn to_yaml(&self) -> Result<String, Error> {
        Yaml::new().dump_str(&self.components)
    }
    pub fn to_json(&self) -> String {
        self.components.to_string()
    }
    pub fn to_component(&self) -> Component {
        Component::new(self.to_value())
    }
    pub fn to_value(&self) -> Value {
        self.components.clone()
    }
}