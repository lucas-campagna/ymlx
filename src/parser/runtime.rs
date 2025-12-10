use indexmap::IndexMap;
use std::ops::Deref;
use super::utils::is_template;
use super::apply::apply;
use super::utils::{get_template_name};
use rust_yaml::{Error, Value};

pub struct Runtime<'a> {
    current_component: Value,
    components: &'a Value,
    call_stack: Vec<String>,
}

impl Deref for Runtime<'_> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl Runtime<'_> {
    pub fn new(components: &Value) -> Runtime<'_> {
        components.as_mapping().expect("Root components should be a JSON");
        Runtime {
            current_component: Value::Null,
            components,
            call_stack: Vec::new(),
         }
    }

    pub fn call(&mut self, name: &str, mut props: Value) -> Result<Value, Error> {
        self.current_component = self.instantiate_component(name);
        self.call_stack.push(name.into());
        self.process_call(&mut props)?;
        self.call_stack.pop();
        Ok(self.current_component.clone())
    }

    fn call_template(&mut self) -> Result<(), Error> {
        if let Some(name) =  self.get_current_component_name() {
            let name = &get_template_name(name);
            self.current_component = self.call(name.as_str(), self.current_component.clone())?;
        }
        Ok(())
    }

    fn process_call(&mut self, props: &mut Value) -> Result<(), Error> {
        let is_template = self.is_current_component_template();
        let has_template = self.has_current_component_template();
        if self.current_component.is_null() && !has_template {
            Ok(())
        } else if is_template {
            apply(&mut self.current_component, props);
            self.parse_from()?;
            self.parse_body()?;
            if has_template {
                self.call_template()
            } else {
                Ok(())
            }
        } else {
            if has_template {
                self.call_template()?;
            }
            apply(&mut self.current_component, props);
            self.parse_from()?;
            self.parse_body()?;
            Ok(())
        }
    }

    fn parse_from(&mut self) -> Result<(), Error> {
        if let Some(from) = self.current_component
            .as_mapping()
            .and_then(|m| m.get(&Value::String("from".into())))
            .cloned()
            &&
            from.is_string()
            &&
            self.components.as_mapping().unwrap().contains_key(&from) {
            self.current_component = self.call(from.as_str().unwrap(), self.current_component.clone())?;
        }
        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), Error> {
        self.current_component = self.parse_body_value(self.current_component.clone())?;
        Ok(())
    }

    fn parse_body_value(&mut self, value: Value) -> Result<Value, Error> {
        eprintln!("parse_body_value {:}", value);
        match value {
            Value::String(name) => {
                if self.components
                    .as_mapping()
                    .unwrap()
                    .contains_key(&Value::String(name.clone()))
                    &&
                    !self.call_stack.contains(&name) {
                    return self.call(&name.clone(), Value::Null);
                }
                Ok(Value::String(name))
            }
            Value::Sequence(mut values) => {
                let mut result = Vec::with_capacity(values.len());
                for value in values.drain(..) {
                    result.push(self.parse_body_value(value)?)
                }
                Ok(Value::Sequence(result))
            }
            Value::Mapping(mut index_map) => {
                let mut result = IndexMap::with_capacity(index_map.len());
                for (key ,value) in index_map.drain(..) {
                    result.insert(key ,self.parse_body_value(value)?);
                }
                Ok(Value::Mapping(result))
            }
            value => {Ok(value)}
        }
    }

    fn get_current_component_name(&self) -> Option<&String> {
        self.call_stack
            .last()
    }
    
    fn instantiate_component(&self, name: &str) -> Value {
        self
            .as_mapping()
            .unwrap()
            .get(&Value::String(name.into()))
            .unwrap_or(&Value::Null)
            .clone()
    }
    
    fn is_current_component_template(&self) -> bool {
        self.get_current_component_name()
            .and_then(|name| Some(is_template(name)))
            .unwrap_or(false)
    }

    fn get_current_component_template(&self) -> Option<&Value> {
        self.get_current_component_name()
            .and_then(|name| {
                self
                    .as_mapping()
                    .unwrap()
                    .get(&Value::String(
                        get_template_name(name.as_str())
                    ))
            })
    }

    fn has_current_component_template(&self) -> bool {
        self.get_current_component_template().is_some()
    }
}
