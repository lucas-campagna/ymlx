use std::collections::HashSet;
use std::ops::Deref;

use super::apply::apply;
use super::utils::{get_template_name, is_template};
use rust_yaml::{Error, Value};

pub struct Runtime<'a>(&'a Value);

impl Deref for Runtime<'_> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Runtime<'_> {
    pub fn new(value: &Value) -> Runtime<'_> {
        Runtime(value)
    }

    fn call_template(&self, name: &str, props: &Value) -> Result<Value, Error> {
        let name = &get_template_name(name);
        self.call(name.as_str(), props)
    }

    pub fn call(&self, name: &str, props: &Value) -> Result<Value, Error> {
        let is_template = is_template(name);
        let name = Value::String(name.into());
        let component = self.0.as_mapping().unwrap().get(&name);
        let has_template = self
            .0
            .as_mapping()
            .unwrap()
            .get(&Value::String(get_template_name(
                name.as_str().unwrap_or_default(),
            )))
            .is_some();
        if component.is_none() && !has_template {
            return Ok(Value::Null);
        }
        let component = component.unwrap_or(&Value::Null);
        if is_template {
            let mut component = apply(component, props);
            self.parse_from(&mut component)?;
            self.parse_body(&mut component)?;
            if has_template {
                return self.call_template(name.as_str().unwrap(), &component);
            }
            Ok(component)
        } else {
            let template = if has_template {
                self.call_template(name.as_str().unwrap(), props)?
            } else {
                Value::Null
            };
            let component = apply(&component, &template);
            let mut component = apply(&component, props);
            self.parse_from(&mut component)?;
            self.parse_body(&mut component)?;
            Ok(component)
        }
    }

    fn parse_from(&self, value: &mut Value) -> Result<(), Error> {
        if let Some(from) = value
            .as_mapping()
            .and_then(|m| m.get(&Value::String("from".into())))
            &&
            from.is_string()
            &&
            self.get_components().contains(from.as_str().unwrap())
        {
            *value = self.call(from.as_str().unwrap(), value)?;
        }
        Ok(())
    }

    fn parse_body(&self, value: &mut Value) -> Result<(), Error> {
        let components = self.get_components();
        fn parse_body_impl(
            runtime: &Runtime,
            value: &mut Value,
            components: &HashSet<String>,
        ) -> Result<(), Error> {
            if let Some(body) = value
                .as_mapping_mut()
                .and_then(|m| m.get_mut(&Value::String("body".into())))
            {
                match body {
                    Value::String(s) => {
                        if components.contains(s) {
                            *value = runtime.call(s, &Value::Null)?;
                        }
                    }
                    Value::Sequence(values) => {
                        for value in values.iter_mut() {
                            parse_body_impl(runtime, value, components)?;
                        }
                    }
                    Value::Mapping(index_map) => {
                        for (_key, value) in index_map.iter_mut() {
                            parse_body_impl(runtime, value, components)?;
                        }
                    }
                    _ => {}
                };
            }
            Ok(())
        }
        parse_body_impl(self, value, &components)
    }

    fn get_components(&self) -> HashSet<String> {
        self.0
            .as_mapping()
            .unwrap()
            .keys()
            .filter_map(|k| {
                if let Value::String(s) = k {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
