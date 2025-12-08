use rust_yaml::Value;
use std::ops::Deref;

use super::constants::IMPLICIT_HTML_COMPONENTS;

pub struct Component(Value);

impl Deref for Component {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Component {
    pub fn new(value: Value) -> Self {
        Component(value)
    }

    pub fn to_json(&self) -> &Value {
        &self.0
    }

    pub fn to_html(&self) -> String {
        fn render(obj: &Value) -> String {
            match obj {
                Value::Mapping(map) => {
                    let mut from = map.get(&Value::String("from".to_string()));
                    let mut body = map.get(&Value::String("body".to_string()));
                    let has_from = from.is_some();
                    let has_body = body.is_some();
                    let props = {
                        let mut result = Vec::new();
                        for (key, value) in map {
                            if !key.is_string() {
                                continue;
                            }
                            let key_str = key.as_str().unwrap().to_string();
                            if has_from && key_str == "from" || has_body && key_str == "body" {
                                continue;
                            }
                            if !has_body
                                && !has_from
                                && IMPLICIT_HTML_COMPONENTS.contains(&key_str.as_str())
                            {
                                from = Some(key);
                                body = Some(value);
                                continue;
                            }
                            result.push(format!("{}=\"{}\"", key_str, render(value)).to_owned());
                        }
                        result
                    };
                    let props = if props.len() == 0 {
                        "".to_owned()
                    } else {
                        " ".to_owned() + &props.join(" ")
                    };
                    match (from, body) {
                        (None, None) => "".to_string(),
                        (None, Some(body)) => render(body),
                        (Some(from), None) => format!(
                            "<{from}{props}></{from}>",
                            from = render(from),
                            props = props
                        ),
                        (Some(from), Some(body)) => format!(
                            "<{from}{props}>{body}</{from}>",
                            from = render(from),
                            body = render(body),
                            props = props
                        ),
                    }
                }
                Value::Sequence(v) => v
                    .iter()
                    .map(|obj| render(obj))
                    .collect::<Vec<String>>()
                    .join(""),
                Value::Null => "".to_string(),
                v => {
                    if v.is_string() {
                        v.as_str().unwrap_or_default().to_string()
                    } else {
                        format!("{}", v)
                    }
                }
            }
        }
        render(&self.0)
    }
}
