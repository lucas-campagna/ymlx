use std::ops::Deref;

use indexmap::IndexMap;

use crate::{discoverable_key::DiscoverableKey, processing_context, utils, value::Value};

pub struct Context(IndexMap<String, Value>);

impl Deref for Context {
    type Target = IndexMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
    pub fn build(index_map: IndexMap<String, Value>) -> Context {
        Context(
            index_map
                .into_iter()
                .map(|(name, mut value)| {
                    if let (name, Some(from)) = utils::extract_call_from_name(&name) {
                        value = match value {
                            Value::Mapping(mut index_map) => {
                                index_map
                                    .insert("from!".to_string(), Value::String(from.to_string()));
                                Value::Mapping(index_map)
                            }
                            value => {
                                let mut index_map = IndexMap::new();
                                index_map
                                    .insert("from!".to_string(), Value::String(from.to_string()));
                                index_map.insert("body".to_string(), value);
                                Value::Mapping(index_map)
                            }
                        };
                        (name.to_string(), value)
                    } else {
                        (name, value)
                    }
                })
                .collect(),
        )
    }
    pub fn call(&self, name: &str, props: Option<&Value>) -> rust_yaml::Value {
        let result = self.inner_call(name, props);
        rust_yaml::Value::from(result)
    }
    fn inner_call(&self, name: &str, props: Option<&Value>) -> Value {
        match (self.0.get(name), props) {
            (Some(c), None) => c.clone(),
            (None, _) => Value::Null,
            (Some(component), Some(props)) => {
                let mut component = component.clone();
                self.apply(&mut component, props);
                self.parse_from(&mut component);
                self.parse_call_template(name, &mut component);
                component
            }
        }
    }
    fn parse_call_template(&self, name: &str, component: &mut Value) {
        if self.has_template(name) {
            let name = format!("${}", name);
            *component = self.inner_call(name.as_str(), Some(component));
        }
    }
    fn parse_from(&self, component: &mut Value) {
        match component {
            Value::Sequence(values) => values.iter_mut().for_each(|v| self.parse_from(v)),
            component => {
                if let Value::Mapping(index_map) = component {
                    index_map.iter_mut().for_each(|(_, value)| {
                        self.parse_from(value);
                    });
                    if let Some(Value::String(name)) =
                        DiscoverableKey::from(&*index_map).get("from")
                    {
                        let name = name.to_owned();
                        let props = Value::Mapping(std::mem::take(index_map));
                        *component = self.inner_call(&name, Some(&props));
                        return;
                    }
                    for name in self.0.keys() {
                        if let Some(props) = DiscoverableKey::from(&*index_map).get(name) {
                            *component = self.inner_call(name, Some(props));
                            return;
                        }
                    }
                }
                if let Value::String(key) = component {
                    if let Some(value) = DiscoverableKey::from(&self.0).clear_and_get(key) {
                        *component = value.clone();
                        return;
                    }
                }
            }
        }
    }
    fn apply(&self, component: &mut Value, props: &Value) {
        match component {
            Value::Sequence(values) => {
                for component in values {
                    self.apply(component, props);
                }
            }
            Value::Mapping(index_map) => {
                for component in index_map.values_mut() {
                    self.apply(component, props);
                }
            }
            component if component.is_string() => {
                let text = component.as_str().unwrap();
                let mut pc = processing_context::ProcessingContext::from(self);
                pc.bind(props);
                *component = pc.parse(&text.to_string());
            }
            _ => {}
        };
    }
    fn has_template(&self, name: &str) -> bool {
        self.0.contains_key(&format!("${}", name))
    }
}
