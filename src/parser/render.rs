use rust_yaml::Value;
use super::constants::IMPLICIT_HTML_COMPONENTS;

pub fn html(value: &Value) -> String {
    match value {
        Value::Mapping(map) => {
            let mut from = map.get(&Value::String("from".to_string()));
            let mut body = map.get(&Value::String("body".to_string()));
            let has_from = from.map_or(false, |v| !v.is_null());
            let has_body = body.map_or(false, |v| !v.is_null());
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
                    result.push(format!("{}=\"{}\"", key_str, html(value)).to_owned());
                }
                result
            };
            let has_from = from.map_or(false, |v| !v.is_null());
            let has_body = body.map_or(false, |v| !v.is_null());
            if !has_body && !has_from {
                return "".to_string();
            }
            let props = if props.len() == 0 {
                "".to_owned()
            } else {
                " ".to_owned() + &props.join(" ")
            };
            match (from, body) {
                (None, None) => "".to_string(),
                (None, Some(body)) => html(body),
                (Some(from), None) => format!(
                    "<{from}{props}></{from}>",
                    from = html(from),
                    props = props
                ),
                (Some(from), Some(body)) => format!(
                    "<{from}{props}>{body}</{from}>",
                    from = html(from),
                    body = html(body),
                    props = props
                ),
            }
        }
        Value::Sequence(v) => v
            .iter()
            .map(|obj| html(obj))
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