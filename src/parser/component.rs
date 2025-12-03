use rust_yaml::Value;

use crate::parser::utils::get_field;

pub struct Component {
    pub from: Option<String>,
    pub body: Value,
    pub properties: Value,
}

impl Component {
    pub fn build(value: &Value) -> Component {
        let from = get_field(value, "from")
            .ok()
            .map(|r| r.to_string());
        let body = get_field(value, "body")
            .ok()
            .map(|r| r.to_owned())
            .unwrap_or(Value::Null);
        let properties = match value {
            Value::Mapping(m) => {
                let mut m = m.to_owned();
                if from.is_some() {
                    m.swap_remove(&Value::String("from".to_owned()));
                }
                m.swap_remove(&Value::String("body".to_owned()));
                Value::Mapping(m)
            },
            v => v.to_owned(),
        };
        Component { from, body, properties }
    }
}