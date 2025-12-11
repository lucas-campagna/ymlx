use std::{collections::HashSet, sync::LazyLock};
use indexmap::IndexMap;
use regex::Regex;
use rust_yaml::Value;

static VAR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap()
});

pub fn clear_props(target: &mut Value) {
    let props: IndexMap<Value, Value> = get_props(target)
        .iter()
        .map(|prop| (Value::String(prop.clone()), Value::Null))
        .collect();
    apply_props(target, &Value::Mapping(props));
}

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

pub fn apply_props(target: &mut Value, source: &Value) {
    let source_map = source.as_mapping().expect("Source should always be mapping!");
    // eprintln!("apply_props source_map {:?}", source_map);
    match target {
        Value::String(target_str) => {
            // eprintln!("apply_props string {}", target_str);
            let get_prop_name = |prop: &str| String::from(&prop[1..]);
            let has_prop_to_apply = source_map.keys().any(|k| {
                match k {
                    Value::String(s) => *s == get_prop_name(target_str),
                    _ => true,
                }
            });
            let prop_value = source_map.get(&Value::String(get_prop_name(target_str)));
            if has_prop_to_apply && let Some(replacement) = prop_value {
                *target = replacement.clone();
                return;
            }
            let result = VAR_RE.replace_all(target_str, |caps: &regex::Captures| {
                let var_name = &caps[1];
                // eprintln!("replace with re {}", var_name);
                if let Some(replacement) = source_map.get(&Value::String(var_name.to_string())) {
                    match replacement {
                        Value::Null => "".to_string(),
                        Value::String(s) => s.to_string(),
                        v => v.to_string(),
                    }
                } else {
                    caps[0].to_string()
                }
            });
            // eprintln!("result: {}", result);
            *target = Value::String(result.trim().to_string())
        }
        target => {
            if matches!(target, Value::Sequence(_)) {
                target
                    .as_sequence_mut()
                    .unwrap()
                    .iter_mut()
                    .for_each(|item| apply_props(item, source));
            }
            if matches!(target, Value::Mapping(_)) {
                target
                    .as_mapping_mut()
                    .unwrap()
                    .values_mut()
                    .for_each(|value| apply_props(value, source));
            }
        }
    }
}

pub fn apply_merge(target: &mut Value, source: &Value) {
    match (target, source) {
        (target, Value::Mapping(source_map))
        if matches!(target, Value::Mapping(..)) => {
            let target_map = target.as_mapping().unwrap();
            let mut merged = target_map.clone();
            for (key, source_value) in source_map.iter() {
                if let Some(target_value) = merged.get(key) {
                    let mut target_value = target_value.clone();
                    apply_merge(&mut target_value, source_value);
                    merged.insert(key.clone(), target_value);
                } else {
                    merged.insert(key.clone(), source_value.clone());
                }
            }
            *target = Value::Mapping(merged)
        }
        (_, Value::Null) => {}
        (target, source) => {
            *target = source.clone();
        }
    }
}

pub fn apply(target: &mut Value, source: &mut Value) {
    eprintln!("Apply to {}  with  {}", target, source);
    if *source == Value::Null {
        return;
    }
    if let Value::Sequence(target_seq) = target
        && let Value::Sequence(source_seq) = source {
        target_seq.append(source_seq);
        return;
    }
    if let Value::Sequence(source_seq) = source {
        let new_target_seq: Vec<Value> = source_seq
            .drain(..)
            .map(|mut source_item| {
                let mut model = target.clone();
                apply(&mut model, &mut source_item);
                model
            })
            .collect();
            *target = Value::Sequence(new_target_seq);
        return;
    }
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
    let common_props = target_props.intersection(&source_props);
    eprintln!("Comon props {:?}", common_props);
    if common_props.count() > 0 {
        eprintln!("Before apply props {} {}", target, source);
        apply_props(target, source);
        // Remove applied props from source
        eprintln!("Before retain {} {}", target, source);
        source
            .as_mapping_mut()
            .unwrap()
            .retain(|k, _| !target_props.contains(k.as_str().unwrap()));
        eprintln!("Before merge {} {}", target, source);
        // Replace remaining props on target
        apply_merge(target, source);
        eprintln!("Apply final {} {}", target, source);
    } else {
        apply_merge(target, source);
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
        let mut target = Yaml::new().load_str(target_yaml).unwrap();
        let mut source = Yaml::new().load_str(source_yaml).unwrap();
        apply(&mut target, &mut source);
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
        assert_eq!(target, expected);
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
        let mut target = Yaml::new().load_str(target_yaml).unwrap();
        let mut source = Yaml::new().load_str(source_yaml).unwrap();
        apply(&mut target, &mut source);
        let expected_yaml = r#"greeting:
    first: "John"
    last: "Doe"
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(target, expected);
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
        let mut target = Yaml::new().load_str(target_yaml).unwrap();
        let mut source = Yaml::new().load_str(source_yaml).unwrap();
        apply(&mut target, &mut source);
        let expected_yaml = r#"list:
  - "Hello, Bob!"
  - "Your age is 30."
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(target, expected);
    }
    
    #[test]
    fn test_apply_props_to_vec() {
        let target_yaml = r#"
list: $items
"#;
        let source_yaml = r#"items:
  - Item1
  - Item2"#;
        let mut target = Yaml::new().load_str(target_yaml).unwrap();
        let mut source = Yaml::new().load_str(source_yaml).unwrap();
        apply(&mut target, &mut source);
        let expected_yaml = r#"list:
  - Item1
  - Item2
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(target, expected);
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
        let mut target = Yaml::new().load_str(target_yaml).unwrap();
        let mut source = Yaml::new().load_str(source_yaml).unwrap();
        apply(&mut target, &mut source);
        let expected_yaml = r#"list: [[Item1, Item2], [Item1, Item2]]"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(target, expected);
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
        let mut target = Yaml::new().load_str(target_yaml).unwrap();
        let mut source = Yaml::new().load_str(source_yaml).unwrap();
        apply(&mut target, &mut source);
        let expected_yaml = r#"
config:
  setting1: "Value1"
  setting2: "NewValue2"
  setting3: "Value3"
"#;
        let expected = Yaml::new().load_str(expected_yaml).unwrap();
        assert_eq!(target, expected);
    }
}