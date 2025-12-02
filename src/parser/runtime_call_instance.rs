use indexmap::IndexMap;
use rust_yaml::Value;
use super::component::Component;
use super::runtime::Runtime;

pub struct RuntimeCallInstance<'a> {
    pub runtime: &'a Runtime,
    pub buffer: Value,
}

impl Runtime {
    fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.get(name)
    }

    fn get_template(&self, name: &str) -> Option<&Component> {
        let name = String::from("$") + name;
        self.components.get(name.as_str())
    }

    fn get_from_of(&self, name: &str) -> Option<String> {
        let comp = self.components.get(name)?;
        if let Some(from) = &comp.from {
            return Some(from.clone());
        }
        let from  = match &comp.properties {
            Value::Mapping(m) => 
                m.keys().find_map(|k| 
                    match k {
                        Value::String(k) if self.components.contains_key(k) => Some(k),
                        Value::String(_) => None,
                        _ => panic!("Invalid property key type"),
                    }
                ),
            Value::Null => None,
            _ => unreachable!()
        };
        if let Some(from) = from {
            return Some(from.clone());
        }
        None
    }
}


impl RuntimeCallInstance<'_> {
    pub fn call<'a>(&'a self, name: &str, params: &Value) {
        let comp = self.runtime.get_component(name);
        self.buffer = if let Some(comp) = comp {
            let mut obj = IndexMap::new();
            if let Some(from) = comp.from {
                obj.insert(Value::String("from".to_string()), from);
            }
            // obj.insert(Value::String("body".to_string()), Value::Int(1));
            obj.insert(Value::Int(1), Value::Int(1));
            
            // for (k,v) in 
            // obj.insert(Value::String("from".to_string()), Value::String("div".to_string()));
            Value::Mapping(obj)
        } else {Value::Null};
        // self.buffer json![];
        // todo!();
        // let is_template = is_template(name);
        // let temp = self.runtime.get_template(name);
    
        // if let Some(temp) = temp {
        //     todo!();
        // }
        // todo!();
    
    
        
        // let has_component = self.components.contains_key(name);
        // let template_name = String::from("$") + name;
        // let has_template = self.templates.contains_key(&template_name);
        // if !has_component && !has_template {
        //     *buff = params.clone();
        // }
        // if is_template {
        //     // let ref mut comp = self.templates.get(name).unwrap();
        //     // while let Some(from) = &comp.from {
        //     //     // self.call(from, )
        //     //     // comp.call(params);
        //     // }
        //     todo!();
        // } else {
        //     if has_template {
        //         todo!();
        //     }
        //     if let Some(comp) = self.components.get(name) {
        //         *buff = self.call_comp(comp, params);
        //     }
        // }
        // self.render()
        // if has_template {
        //     if is_template {
    
        //     } else {
        //     }
        // }
    }
    
    fn call_from(&self, comp: &Component, params: &Value) -> Value {
        if let Some(from) = &comp.from {
            self.call(from, params);
            // let comp = self.runtime.get_component(from);
            // let temp = self.runtime.get_template(from);
            // return Value::Null;
            todo!();
        }
        return Value::Null;
    }
    
}

fn is_template(name: &str) -> bool{
    name.starts_with("$")
}