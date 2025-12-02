use rust_yaml::Value;


pub fn render(obj: &Value) -> String {
    let s = |s| String::from(s);
    let k = |k| Value::String(s(k));
    match obj {
        Value::Mapping(m) => {
            if let Some(from) = m.get(&k("from")) {
                let body = m
                    .get(&k("from"))
                    .and_then(|k| Some(k.to_string()))
                    .unwrap_or(s("body"));
                let props = {
                    let mut result = String::new();
                    for (k, v) in m {
                        fn check_is_valid_type(k: &Value) -> bool {
                            k.is_string() || k.is_bool() || k.is_number()
                        }
                        let is_prop = check_is_valid_type(k) && check_is_valid_type(v);
                        if is_prop {
                            result.push_str(format!("{}={}", k, v).as_str());
                        }
                    }
                    result
                };
                return format!("<{from} {props}>{body}</{from}>", from=from, body=body, props=props);
            }
            String::from("")
        },
        Value::Sequence(v) => v.iter().map(|obj| render(obj)).collect::<Vec<String>>().join(""),
        v => v.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::{Input, build};
    use super::*;

    #[test]
    fn render_with_from_and_body() {
        let code = r#"
box:
    from: div
    body: hello world
    "#;
        let rt = build(Input::Code(code)).unwrap();
        let comp = rt.call("box", None);
        assert_eq![render(&comp), "<div>hello world</div>".to_string()];
    }
}
