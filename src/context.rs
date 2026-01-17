use indexmap::IndexMap;
use std::ops::{Deref, DerefMut};

use crate::{discoverable_key::DiscoverableKey, merge::ComponentMerger, value::Value};

pub struct Context(IndexMap<String, Value>);

impl Deref for Context {
    type Target = IndexMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Context {
    pub fn build(index_map: IndexMap<String, Value>) -> Context {
        let mut merger = ComponentMerger::with_capacity(index_map.len());
        let mut context: Context = Context(
            index_map
                .into_iter()
                .map(|(name, value)| (merger.parse_name(name), value))
                .collect(),
        );
        merger.parse(&mut context);
        context
    }

    pub fn call(&self, name: &str, props: Option<&Value>) -> Value {
        match (self.get(name), props) {
            (Some(component), None) => component.clone(),
            (None, _) => Value::Null,
            (Some(component), Some(props)) => {
                let mut component = component.clone();
                component.apply(props, &Some(self));
                self.parse_from(&mut component);
                component
            }
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
                        *component = self.call(&name, Some(&props));
                        return;
                    }
                    for name in self.0.keys() {
                        if let Some(body) = DiscoverableKey::from(&*index_map).get(name) {
                            let body = body.to_owned();
                            index_map.swap_remove(name);
                            index_map.insert("body".to_owned(), body);
                            let props = Value::Mapping(std::mem::take(index_map));
                            *component = self.call(name, Some(&props));
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
}
