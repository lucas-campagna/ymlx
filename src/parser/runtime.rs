use indexmap::IndexMap;
use std::ops::Deref;
use super::utils::is_template;
use super::apply::apply;
use super::utils::{get_template_name};
use rust_yaml::{Error, Value};
use super::constants::IMPLICIT_HTML_COMPONENTS;

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
        if self.call_stack.contains(&name.to_string()) {
            return Ok(props);
        }
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
            eprintln!("Processing call to template {:?}", self.get_current_component_name());
            eprintln!("Before apply props {}", self.current_component);
            apply(&mut self.current_component, props);
            self.parse_component()?;
            if has_template {
                self.call_template()
            } else {
                Ok(())
            }
        } else {
            eprintln!("Processing call to component {:?}", self.get_current_component_name());
            if has_template {
                self.call_template()?;
            }
            eprintln!("Before apply props {}", self.current_component);
            apply(&mut self.current_component, props);
            self.parse_component()?;
            Ok(())
        }
    }
    
    /// Apply the `shortcut`, `from` and `composition` parsers in the correct order
    fn parse_component(&mut self) -> Result<(), Error> {
        eprintln!("Before parse shortcut {}", self.current_component);
        self.parse_shortcut()?;
        eprintln!("Before parse from {}", self.current_component);
        self.parse_from()?;
        eprintln!("Before parse composition {}", self.current_component);
        self.parse_composition()?;
        eprintln!("Final {}", self.current_component);
        Ok(())
    }

    /// Any component with `from` property pointing to another valid component is a call to that component sending the other properties as parameter.
    /// This function process this call.
    /// Example:
    /// 
    /// ```yaml
    /// app:
    ///     from: my_button
    ///     text: Hello World!
    /// my_button:
    ///     onclick: alert('clicked')
    ///     body: $text
    /// ```
    /// 
    /// After calling this function it will produces:
    /// 
    /// ```yaml
    /// app:
    ///     onclick: alert('clicked')
    ///     body: Hello World!
    /// ```
    /// 
    /// This is recursivelly applied from inner first
    fn parse_from(&mut self) -> Result<(), Error> {
        if self.current_component.is_mapping() || self.current_component.is_sequence() {
            self.current_component = self.parse_from_value(self.current_component.clone())?;
        }
        Ok(())
    }

    fn parse_from_value(&mut self, value: Value) -> Result<Value, Error> {
        match value {
            Value::Sequence(value_seq) => {
                eprintln!("Is Seq");
                let result = value_seq
                    .into_iter()
                    .map(|value| self.parse_from_value(value))
                    .collect::<Result<Vec<Value>, Error>>()?;
                let result = Value::Sequence(result);
                eprintln!("parse_from_value Final: {}", result.to_string());
                Ok(result)
            }
            Value::Mapping(mut value_map) => {
                eprintln!("Is Map");
                value_map = value_map
                    .into_iter()
                    .map(|(key, value)| -> Result<(Value, Value), Error> {
                        if key == Value::String("from".into()) {
                            Ok((key, value))
                        } else {
                            Ok((key, self.parse_from_value(value)?))
                        }
                    })
                    .collect::<Result<IndexMap<Value, Value>, Error>>()?;
                
                let key_from = Value::String("from".into());
                let result = if let Some(Value::String(from)) = value_map.get(&key_from)
                    && self.has_component_or_template(from) {
                    eprintln!("Has from");
                    let from = from.clone();
                    value_map.swap_remove(&key_from);
                    let props = self.parse_from_value(Value::Mapping(value_map))?;
                    eprintln!("Calling {}", from);
                    self.call(&from, props)?
                } else {
                    Value::Mapping(value_map)
                };
                Ok(result)
            }
            v => Ok(v)
        }
    }

    /// This parses shortcuts for `from` and `body`
    /// Example:
    /// 
    /// ```yaml
    /// app:
    ///     div: Hello World!
    ///     prop: value
    /// ```
    /// 
    /// This is a shortcut for
    /// 
    /// ```yaml
    /// app:
    ///     from: div
    ///     body: Hello World!
    ///     prop: value
    /// ```
    fn parse_shortcut(&mut self) -> Result<(), Error> {
        if self.current_component.is_mapping() || self.current_component.is_sequence() {
            self.current_component = self.parse_shortcut_value(self.current_component.clone())?;
        }
        Ok(())
    }

    fn parse_shortcut_value(&mut self, value: Value) -> Result<Value, Error> {
        println!("parse_shortcut_value {}", value);
        match value {
            Value::Sequence(mut value_seq) => {
                let mut result = Vec::with_capacity(value_seq.len());
                for value in value_seq.drain(..) {
                    result.push(self.parse_shortcut_value(value)?)
                }
                let result = Value::Sequence(result);
                eprintln!("parse_shortcut_value Final: {}", result.to_string());
                Ok(result)
            },
            Value::Mapping(mut value_map) => {
                let from = value_map.get(&Value::String("from".into()));
                let body = value_map.get(&Value::String("body".into()));
                let has_from = from.map_or(false, |v| *v != Value::Null);
                let has_body = body.map_or(false, |v| *v != Value::Null);
                eprintln!("Is Mapping from: {:?} body: {:?}", from, body);
                if has_from || has_body {
                    for value in value_map.values_mut() {
                        *value = self.parse_shortcut_value(value.to_owned())?;
                    }
                    let result = Value::Mapping(value_map);
                    eprintln!("parse_shortcut_value Final: {}", result.to_string());
                    return Ok(result);
                }
                let key_value = value_map
                    .iter()
                    .find(|(key, _)| {
                        if let Value::String(key) = key {
                            return IMPLICIT_HTML_COMPONENTS.contains(&key.as_str())
                                || self.has_component_or_template(key);
                        }
                        false
                    });
                if let Some((key, value)) = key_value {
                    let key = key.clone();
                    let value = value.clone();
                    value_map.swap_remove(&key);
                    value_map.insert(Value::String("from".into()), key);
                    value_map.insert(Value::String("body".into()), value);
                }
                let result = Value::Mapping(value_map);
                eprintln!("parse_shortcut_value Final: {}", result.to_string());
                Ok(result)
            },
            value => Ok(value),
        }
    }
    
    /// Composition is the reference to one component inside another component
    /// Example:
    /// 
    /// ```yaml
    /// app: button
    /// button: My Button
    /// ```
    /// 
    /// `button` component is been refereced from `app`'s body
    fn parse_composition(&mut self) -> Result<(), Error> {
        self.current_component = self.parse_composition_value(self.current_component.clone())?;
        Ok(())
    }

    fn parse_composition_value(&mut self, value: Value) -> Result<Value, Error> {
        eprintln!("parse_composition_value {:}", value);
        match value {
            Value::String(name) => {
                eprintln!("Is String");
                let result = if self.components_map()
                    .contains_key(&Value::String(name.clone()))
                    &&
                    !self.call_stack.contains(&name) {
                    eprintln!("Calling {}", name);
                    self.call(&name, Value::Null)?
                } else {
                    Value::String(name)
                };
                Ok(result)
            }
            Value::Sequence(value_seq) => {
                eprintln!("Is Sequence");
                let result = value_seq
                    .into_iter()
                    .map(|value| self.parse_composition_value(value))
                    .collect::<Result<Vec<Value>, Error>>()?;
                eprintln!("Seq Result: {:?}", result);
                Ok(Value::Sequence(result))
            }
            Value::Mapping(mut index_map) => {
                let key_from = Value::String("from".into());
                eprintln!("Is Mapping ({:?})", self.components_map().keys());
                if let Some(Value::String(from)) = index_map.get(&key_from)
                    && self.has_component_or_template(from) {
                    eprintln!("Has from");
                    let from = from.clone();
                    index_map.swap_remove(&key_from);
                    let props = Value::Mapping(index_map);
                    eprintln!("Calling {}", from);
                    let result = self.call(&from, props)?;
                    Ok(result)
                } else {
                    eprintln!("Do not has from");
                    let result = index_map
                        .into_iter()
                        .map(|(key, value)| -> Result<(Value, Value), Error> {
                            let result = if !key.is_string() || key == Value::String("from".into()) {
                                (key, value)
                            } else {
                                (key, self.parse_composition_value(value)?)
                            };
                            Ok(result)
                        })
                        .collect::<Result<IndexMap<Value, Value>, Error>>()?;
                    Ok(Value::Mapping(result))
                }
            }
            value => Ok(value)
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

    fn components_map(&self) -> &IndexMap<Value, Value> {
        self.components
            .as_mapping()
            .unwrap()
    }

    fn current_component_map(&self) -> &IndexMap<Value, Value> {
        self.current_component
            .as_mapping()
            .unwrap()
    }
    fn current_component_map_mut(&mut self) -> &mut IndexMap<Value, Value> {
        self.current_component
            .as_mapping_mut()
            .unwrap()
    }

    fn has_component_or_template(&self, name: &str) -> bool {
        let components = self.components_map();
        let template_name = Value::String("$".to_string() + name);
        let name = Value::String(name.into());
        components.contains_key(&name)
            || components.contains_key(&template_name)
    }
}
