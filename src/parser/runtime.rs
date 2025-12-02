use std::collections::HashMap;
use rust_yaml::Value;
use super::component::Component;
use super::runtime_call_instance::RuntimeCallInstance;

pub struct Runtime {
    pub components: HashMap<String, Component>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            components: HashMap::new()
        }
    }

    pub fn add_many(&mut self, comps: &Value) -> Result<(), ()>{
        match comps {
            Value::Mapping(comps) => {
                for (name, value) in comps.iter() {
                    let name = match name {
                        Value::String(name) => name,
                        _ => panic!("Invalid component name type"),
                    };
                    self.add(name, value);
                };
                Ok(())
            },
            _ => Err(())
        }
    }

    pub fn call(&self, name: &str, params: Option<&Value>) -> Box<Value> {
        let params = params.unwrap_or(&Value::Null);
        let call_instance = RuntimeCallInstance {
            runtime: self,
            buffer: params.clone(),
        };
        call_instance.call(name, params);
        Box::new(call_instance.buffer)
    }

    pub fn add(&mut self, name: &str, comp: &Value) {
        self.components.insert(name.to_owned(), Component::build(comp));
    }
}