use rust_yaml::Value;

pub struct Component {
    pub from: Option<String>,
    pub body: Value,
    pub properties: Value,
}

impl Component {
    pub fn build(value: &Value) -> Component {
        let key_from =  Value::String(String::from("from"));
        let key_body =  Value::String(String::from("body"));
        let from = match value {
            Value::Mapping(m) =>
                m.get(&key_from)
                    .and_then(|v| Some(v.to_string())),
            _ => None
            };
        let body = match value {
            Value::Mapping(m) =>
                m.get(&key_body)
                    .and_then(|v| Some(Value::String(v.to_string())))
                    .or(Some(Value::Null)),
            v => Some(v.to_owned()),
        }.unwrap();
        let properties = match value {
            Value::Mapping(m) => {
                let mut m = m.to_owned();
                m.swap_remove(&key_body);
                m.swap_remove(&key_from);
                Value::Mapping(m)
            },
            _ => Value::Null,
        };
        Component { from, body, properties }
    }
}