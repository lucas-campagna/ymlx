use std::{collections::HashSet, sync::LazyLock};
use regex::Regex;
use rust_yaml::Value;

static VAR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap()
});

pub fn get_props(comp: &Value) -> Vec<String> {
    match comp {
        Value::String(s) => VAR_RE
            .captures_iter(s)
            .map(|mat| mat.get(1).unwrap().as_str().to_string())
            .collect(),
        Value::Sequence(values) => values.iter().flat_map(get_props).collect(),
        Value::Mapping(index_map) => index_map.values().flat_map(get_props).collect(),
        _ => vec![],
    }
}

pub fn apply_props(target: &Value, source: &Value) -> Value {
    match (target, source) {
        (Value::String(target_str), Value::Mapping(source_map)) => {
            if source_map.keys().any(|k| {
                match k {
                    Value::String(s) => s == &target_str[1..],
                    _ => true,
                }
            }) && let Some(replacement) = source_map.get(&Value::String(target_str[1..].to_string())){
                return replacement.clone();
            }
            let result = VAR_RE.replace_all(target_str, |caps: &regex::Captures| {
                let var_name = &caps[1];
                if let Some(replacement) = source_map.get(&Value::String(var_name.to_string())) &&
                    let Value::String(repl_str) = replacement {
                    repl_str.clone()
                } else {
                    caps[0].to_string()
                }
            });
            Value::String(result.into_owned())
        }
        (Value::Sequence(target_seq), Value::Mapping(_)) => {
            let new_seq: Vec<Value> = target_seq
                .iter()
                .map(|item| apply_props(item, source))
                .collect();
            Value::Sequence(new_seq)
        }
        (Value::Mapping(target_map), Value::Mapping(_)) => {
            let mut new_map = target_map.clone();
            for (key, value) in target_map {
                let new_value = apply_props(value, source);
                new_map.insert(key.clone(), new_value);
            }
            Value::Mapping(new_map)
        }
        _ => target.clone(),
    }
}

pub fn apply_merge(target: &Value, source: &Value) -> Value {
    match (target, source) {
        (Value::Mapping(target_map), Value::Mapping(source_map)) => {
            let mut merged = target_map.clone();
            for (key, source_value) in source_map {
                if let Some(target_value) = merged.get(key) {
                    merged.insert(key.clone(), apply_merge(target_value, source_value));
                } else {
                    merged.insert(key.clone(), source_value.clone());
                }
            }
            Value::Mapping(merged)
        },
        (_, Value::Null) => target.clone(),
        (Value::Null, _) => Value::Null,
        _ => source.clone(),
    }
}

pub fn apply(target: &Value, source: &Value) -> Value {
    let target_props: HashSet<String> = get_props(target).into_iter().collect();
    let source_props: HashSet<String> = match source {
        Value::Mapping(map) => map.keys().filter_map(|k| {
            if let Value::String(s) = k {
                Some(s.clone())
            } else {
                None
            }
        }).collect(),
        _ => vec![],
    }.into_iter().collect();
    if target_props.intersection(&source_props).count() > 0 {
        apply_props(target, source)
    } else {
        apply_merge(target, source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_yaml::Yaml;
    #[test]
    fn test_get_props() {
        let yaml_str = r#"
key1: "$var1 and $var2"
key2:
  - "no vars here"
  - "$var3 is here"
key3:
  subkey1: "$var4"
  subkey2: "also $var5"
"#;
        let value = Yaml::new().load_str(yaml_str).unwrap();
        let props = get_props(&value);
        let mut expected = vec!["var1".to_string(), "var2".to_string(), "var3".to_string(), "var4".to_string(), "var5".to_string()];
        expected.sort();
        let mut props_sorted = props;
        props_sorted.sort();
        assert_eq!(props_sorted, expected);
    }

    #[test]
    fn test_apply_props() {
        let target_yaml = r#"
message: "Hello, $name!"
items:
  - "Item 1: $item1"
  - "Item 2: $item2"
config:
  setting1: "$setting1"
  setting2: "Value without vars"
"#;
        let source_yaml = r#"
name: "Alice"
item1: "Book"
item2: "Pen"
setting1: "Enabled"
"#;
        let target = Yaml::new().load_str(target_yaml).unwrap();
        let source = Yaml::new().load_str(source_yaml).unwrap();
        let result = apply(&target, &source);
        let expected_yaml = r#"
message: "Hello, Alice!"
items:
  - "Item 1: Book"
  - "Item 2: Pen"
config:
  setting1: "Enabled"
  setting2: "Value without vars"
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_props_object() {
        let target_yaml = r#"
greeting: $user
"#;
        let source_yaml = r#"
user:
    first: "John"
    last: "Doe"
"#;
        let target = Yaml::new().load_str(target_yaml).unwrap();
        let source = Yaml::new().load_str(source_yaml).unwrap();
        let result = apply(&target, &source);
        let expected_yaml = r#"greeting:
    first: "John"
    last: "Doe"
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_props_with_vec() {
        let target_yaml = r#"
list:
  - "Hello, $name!"
  - "Your age is $age."
"#;
        let source_yaml = r#"name: "Bob"
age: "30""#;
        let target = Yaml::new().load_str(target_yaml).unwrap();
        let source = Yaml::new().load_str(source_yaml).unwrap();
        let result = apply(&target, &source);
        let expected_yaml = r#"list:
  - "Hello, Bob!"
  - "Your age is 30."
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_apply_props_to_vec() {
        let target_yaml = r#"
list: $items
"#;
        let source_yaml = r#"items:
  - Item1
  - Item2"#;
        let target = Yaml::new().load_str(target_yaml).unwrap();
        let source = Yaml::new().load_str(source_yaml).unwrap();
        let result = apply(&target, &source);
        let expected_yaml = r#"list:
  - Item1
  - Item2
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_apply_props_to_vecs() {
        let target_yaml = r#"
list:
  - $items
  - $items
"#;
        let source_yaml = r#"items:
  - Item1
  - Item2"#;
        let target = Yaml::new().load_str(target_yaml).unwrap();
        let source = Yaml::new().load_str(source_yaml).unwrap();
        let result = apply(&target, &source);
        let expected_yaml = r#"list: [[Item1, Item2], [Item1, Item2]]"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_apply_merge() {
        let target_yaml = r#"
config:
  setting1: "Value1"
  setting2: "Value2"
"#;
        let source_yaml = r#"
config:
  setting2: "NewValue2"
  setting3: "Value3"
"#;
        let target = Yaml::new().load_str(target_yaml).unwrap();
        let source = Yaml::new().load_str(source_yaml).unwrap();
        let result = apply(&target, &source);
        let expected_yaml = r#"
config:
  setting1: "Value1"
  setting2: "NewValue2"
  setting3: "Value3"
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(result, expected);
    }
}