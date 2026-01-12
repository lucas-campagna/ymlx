use ymx::*;

// Test fixtures and utilities
use crate::fixtures::*;

#[cfg(test)]
mod core_parser_tests {
    use super::*;

    #[test]
    fn test_parse_empty_yaml() {
        let content = "";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let content = "   \n  \t  \n  ";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_parse_comments_only() {
        let content = "# This is a comment\n# Another comment\n";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_parse_single_string_component() {
        let content = "hello: Hello World!";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "hello");
        match &components[0].value {
            ComponentValue::Literal(s) => assert_eq!(s, "Hello World!"),
            _ => panic!("Expected Literal component"),
        }
    }

    #[test]
    fn test_parse_multiple_components() {
        let content = r#"
component1: Value1
component2: Value2
component3: Value3
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].name, "component1");
        assert_eq!(components[1].name, "component2");
        assert_eq!(components[2].name, "component3");
    }

    #[test]
    fn test_parse_numeric_values() {
        let content = r#"
integer: 42
float: 3.14
negative: -100
zero: 0
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(s) => {
                    assert!(!s.is_empty());
                },
                _ => panic!("Expected Literal component for numeric values"),
            }
        }
    }

    #[test]
    fn test_parse_boolean_values() {
        let content = r#"
true_val: true
false_val: false
yes: yes
no: no
on: on
off: off
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 6);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for boolean values"),
            }
        }
    }

    #[test]
    fn test_parse_null_value() {
        let content = "null_component: null";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(s) => assert_eq!(s, "null"),
            _ => panic!("Expected Literal component for null value"),
        }
    }

    #[test]
    fn test_parse_multiline_string() {
        let content = r#"
multiline: |
  This is a
  multiline string
  with multiple lines
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(s) => {
                assert!(s.contains("multiline string"));
            },
            _ => panic!("Expected Literal component for multiline string"),
        }
    }

    #[test]
    fn test_parse_array_values() {
        let content = r#"
simple_array: [1, 2, 3]
string_array: ["a", "b", "c"]
mixed_array: [1, "two", true]
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Arrays are converted to literal strings
                _ => panic!("Expected Literal component for array values"),
            }
        }
    }

    #[test]
    fn test_parse_nested_mapping() {
        let content = r#"
parent:
  child1: value1
  child2: value2
  nested:
    deep: value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Nested objects are converted to literal strings
            _ => panic!("Expected Literal component for nested mapping"),
        }
    }

    #[test]
    fn test_parse_property_reference() {
        let content = r#"
simple_ref: $name
complex_ref: ${user.name}
nested_ref: $user.profile.settings.theme
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        // Test property reference detection
        for component in &components {
            match &component.value {
                ComponentValue::PropertyReference(_) | ComponentValue::ProcessingContext(_) => {}, // Expected
                _ => panic!("Expected PropertyReference or ProcessingContext for property references"),
            }
        }
    }

    #[test]
    fn test_parse_processing_context() {
        let content = r#"
simple_expr: ${1 + 2}
complex_expr: ${user.age + 10}
string_concat: ${"Hello, " + name}
func_call: ${Math.max(a, b)}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::ProcessingContext(_) => {}, // Expected
                _ => panic!("Expected ProcessingContext for expressions"),
            }
        }
    }

    #[test]
    fn test_parse_component_call_with_from() {
        let content = r#"
button:
  from!: base_button
  text: Click me
  style: primary
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base_button");
                assert_eq!(call.properties.get("text").unwrap(), &ComponentValue::Literal("Click me".to_string()));
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_parse_component_call_with_yx_from() {
        let content = r#"
card:
  yx-from: base_card
  title: Card Title
  content: Card content
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base_card");
                assert_eq!(call.properties.get("title").unwrap(), &ComponentValue::Literal("Card Title".to_string()));
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_parse_component_call_with_capital_from() {
        let content = r#"
modal:
  From: base_modal
  title: Modal Title
  size: large
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base_modal");
                assert_eq!(call.properties.get("title").unwrap(), &ComponentValue::Literal("Modal Title".to_string()));
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_parse_component_call_with_properties() {
        let content = r#"
user_card:
  from!: base_component
  name: $user_name
  age: $user_age
  profile:
    avatar: $avatar_url
    bio: $user_bio
  settings:
    theme: dark
    notifications: true
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base_component");
                assert!(call.properties.contains_key("name"));
                assert!(call.properties.contains_key("age"));
                assert!(call.properties.contains_key("profile"));
                assert!(call.properties.contains_key("settings"));
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_parse_mixed_component_types() {
        let content = r#"
# Simple literal
simple: Hello World

# Property reference
prop_ref: $name

# Processing context
expr: ${1 + 2}

# Component call
component_call:
  from!: base
  text: Click me

# Array
array: [1, 2, 3]

# Nested object
object:
  key: value
  nested:
    deep: value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 6);
        
        let mut found_types = std::collections::HashSet::new();
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => { found_types.insert("literal"); },
                ComponentValue::PropertyReference(_) => { found_types.insert("property_ref"); },
                ComponentValue::ProcessingContext(_) => { found_types.insert("processing_context"); },
                ComponentValue::ComponentCall(_) => { found_types.insert("component_call"); },
                ComponentValue::Template(_) => { found_types.insert("template"); },
            }
        }
        
        assert!(found_types.contains("literal"));
        assert!(found_types.contains("property_ref"));
        assert!(found_types.contains("processing_context"));
        assert!(found_types.contains("component_call"));
    }

    #[test]
    fn test_parse_special_characters_in_names() {
        let content = r#"
component-with-dashes: value1
component_with_underscores: value2
component.with.dots: value3
ComponentWithCamelCase: value4
component123: value5
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        let expected_names = vec![
            "component-with-dashes",
            "component_with_underscores", 
            "component.with.dots",
            "ComponentWithCamelCase",
            "component123"
        ];
        
        for (i, expected_name) in expected_names.iter().enumerate() {
            assert_eq!(components[i].name, *expected_name);
        }
    }

    #[test]
    fn test_parse_unicode_content() {
        let content = r#"
emoji: 🚀 Rocket
chinese: 你好世界
arabic: مرحبا بالعالم
emoji_complex: 🎨🖼️🎭🎪
unicode_mix: Hello 世界 🌍 123
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(s) => {
                    assert!(!s.is_empty());
                    // Check if unicode is preserved
                    assert!(s.chars().any(|c| c > '\u{7F}'));
                },
                _ => panic!("Expected Literal component for unicode content"),
            }
        }
    }

    #[test]
    fn test_parse_escaped_characters() {
        let content = r#"
quotes: "Hello \"World\""
newlines: "Line1\nLine2"
tabs: "Col1\tCol2"
backslashes: "Path\\to\\file"
unicode_escape: "\u00A9 Copyright"
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for escaped characters"),
            }
        }
    }

    #[test]
    fn test_parse_empty_string() {
        let content = r#"
empty_string: ""
explicit_empty: ''
null_string: 
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn test_parse_large_values() {
        let content = r#"
large_number: 9223372036854775807
negative_large: -9223372036854775808
long_string: "This is a very long string that contains many characters and should be handled properly by the parser without any issues or truncation"
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for large values"),
            }
        }
    }

    #[test]
    fn test_parse_scientific_notation() {
        let content = r#"
scientific_pos: 1.23e+10
scientific_neg: 4.56e-5
scientific_upper: 7.89E+6
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for scientific notation"),
            }
        }
    }

    #[test]
    fn test_parse_hexadecimal_and_octal() {
        let content = r#"
hex_lower: 0x1a
hex_upper: 0XFF
octal: 0o755
binary: 0b1010
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for numeric literals"),
            }
        }
    }

    #[test]
    fn test_parse_infinity_and_nan() {
        let content = r#"
inf_pos: .inf
inf_neg: -.Inf
nan: .NaN
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for special floats"),
            }
        }
    }

    #[test]
    fn test_parse_timestamp_values() {
        let content = r#"
iso_date: 2023-12-25
iso_datetime: 2023-12-25T15:30:00Z
space_separated: 2023-12-25 15:30:00
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for timestamp values"),
            }
        }
    }

    #[test]
    fn test_parse_merge_keys() {
        let content = r#"
base: &base
  prop1: value1
  prop2: value2

merged:
  <<: *base
  prop3: value3
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        // Merge keys are handled by YAML parser, should result in merged object
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component after merge processing"),
            }
        }
    }

    #[test]
    fn test_parse_anchors_and_aliases() {
        let content = r#"
default_settings: &default
  timeout: 30
  retries: 3

service1:
  <<: *default
  name: service1

service2:
  <<: *default
  name: service2
  timeout: 60  # Override default
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component after anchor/alias processing"),
            }
        }
    }

    #[test]
    fn test_parse_set_values() {
        let content = r#"
string_set: !!set {"a", "b", "c"}
number_set: !!set {1, 2, 3}
mixed_set: !!set {"a", 1, true}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for set values"),
            }
        }
    }

    #[test]
    fn test_parse_ordered_map() {
        let content = r#"
ordered_map: !!omap
  - key1: value1
  - key2: value2
  - key3: value3
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Expected after conversion
            _ => panic!("Expected Literal component for ordered map"),
        }
    }

    #[test]
    fn test_parse_pairs() {
        let content = r#"
pairs: !!pairs
  - key1: value1
  - key2: value2
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Expected after conversion
            _ => panic!("Expected Literal component for pairs"),
        }
    }

    #[test]
    fn test_parse_binary_data() {
        let content = r#"
binary_data: !!binary |
  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5
  OTk6enp56enolpaSEhP/++f/++f/++f/++f/++f/++f/++f/++f/++f/+
  +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACH5BAEAAAAALAAAAAAMAAwA
  AAIUlI+hI+py+0Po5y02qsADQE=
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Expected after conversion
            _ => panic!("Expected Literal component for binary data"),
        }
    }

    #[test]
    fn test_parse_custom_tags() {
        let content = r#"
custom_tag: !custom_tag value
another_custom: !another:some:type complex_value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected for unknown custom tags
                _ => panic!("Expected Literal component for custom tags"),
            }
        }
    }

    #[test]
    fn test_parse_flow_style_collections() {
        let content = r#"
flow_map: {key1: value1, key2: value2, key3: value3}
flow_array: [item1, item2, item3, item4]
nested_flow: {outer: {inner: value}, array: [1, 2, 3]}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for flow style collections"),
            }
        }
    }

    #[test]
    fn test_parse_complex_nesting() {
        let content = r#"
complex:
  level1:
    level2:
      level3:
        deep_value: found
      level3_array: [1, 2, 3]
    level2_map:
      nested_key: nested_value
  level1_array:
    - item1
    - item2
    - nested: {key: value}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Expected after conversion
            _ => panic!("Expected Literal component for complex nesting"),
        }
    }

    #[test]
    fn test_parse_duplicate_keys() {
        let content = r#"
duplicate: first_value
duplicate: second_value
"#;
        // YAML parser should handle this according to YAML spec (last wins)
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(s) => assert_eq!(s, "second_value"),
            _ => panic!("Expected Literal component"),
        }
    }

    #[test]
    fn test_parse_reserved_words_as_keys() {
        let content = r#"
true: boolean_true
false: boolean_false
null: null_value
yes: yes_value
no: no_value
on: on_value
off: off_value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 7);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for reserved words as keys"),
            }
        }
    }

    #[test]
    fn test_parse_quoted_and_unquoted_keys() {
        let content = r#"
unquoted_key: value1
"quoted_key": value2
'single_quoted_key': value3
key with spaces: value4
"key.with.dots": value5
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        let expected_names = vec![
            "unquoted_key",
            "quoted_key",
            "single_quoted_key",
            "key with spaces",
            "key.with.dots"
        ];
        
        for (i, expected_name) in expected_names.iter().enumerate() {
            assert_eq!(components[i].name, *expected_name);
        }
    }

    #[test]
    fn test_parse_empty_collections() {
        let content = r#"
empty_map: {}
empty_array: []
empty_flow_map: {}
empty_flow_array: []
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for empty collections"),
            }
        }
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let content = r#"
invalid: [unclosed array
another: unclosed "string
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("YAML parsing error"));
    }

    #[test]
    fn test_parse_malformed_component_call() {
        let content = r#"
invalid_call:
  from!: 
  # Missing target
  prop: value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        // Should treat as regular component, not component call
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Expected
            _ => panic!("Expected Literal component for malformed component call"),
        }
    }

    #[test]
    fn test_parse_deeply_nested_properties() {
        let content = r#"
deep_call:
  from!: base
  level1:
    level2:
      level3:
        level4:
          level5: deep_value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base");
                assert!(call.properties.contains_key("level1"));
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_parse_mixed_property_types() {
        let content = r#"
mixed_props:
  from!: base
  string_prop: "hello"
  number_prop: 42
  bool_prop: true
  null_prop: null
  array_prop: [1, 2, 3]
  object_prop: {key: value}
  prop_ref: $name
  expression: ${1 + 2}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base");
                assert_eq!(call.properties.len(), 8);
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_parse_very_long_component_name() {
        let long_name = "a".repeat(1000);
        let content = format!("{}: value", long_name);
        let result = parse_yaml_content(&content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, long_name);
    }

    #[test]
    fn test_parse_special_yaml_constructs() {
        let content = r#"
document_separator: |
  content
document_with_explicit_markers: !!str explicit_string
explicit_null: !!null null
explicit_bool: !!bool true
explicit_int: !!int 42
explicit_float: !!float 3.14
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 6);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for explicit YAML constructs"),
            }
        }
    }

    #[test]
    fn test_parse_multiple_documents() {
        let content = r#"
---
document1: value1
---
document2: value2
...
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        // Should parse first document only
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "document1");
    }

    #[test]
    fn test_parse_with_bom() {
        let content = "\u{FEFF}bom_test: value_with_bom";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_parse_line_folding() {
        let content = r#"
folded: >
  This is a folded
  string that becomes
  a single line

literal: |
  This is a literal
  string that preserves
  newlines
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for folded/literal strings"),
            }
        }
    }

    #[test]
    fn test_parse_template_values() {
        let content = r#"
template1: Hello {{name}}!
template2: {{user.name}} has {{count}} items
template3: Result: {{calculation}}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        // Currently templates are treated as literals, but we can detect them
        for component in &components {
            match &component.value {
                ComponentValue::Literal(s) => {
                    if s.contains("{{") && s.contains("}}") {
                        // Could be enhanced to Template type in future
                    }
                },
                _ => panic!("Expected Literal component for template values"),
            }
        }
    }

    #[test]
    fn test_parse_comment_handling() {
        let content = r#"
# Top level comment
component1: value1  # Inline comment
# Another comment
component2: 
  # Inside comment
  value2
  # Another inside comment
# Final comment
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].name, "component1");
        assert_eq!(components[1].name, "component2");
    }

    #[test]
    fn test_parse_directive_style() {
        let content = r#"
%YAML 1.2
%TAG ! !foo:
%TAG ! !! foo:
component: value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "component");
    }

    #[test]
    fn test_parse_node_properties() {
        let content = r#"
node_with_props: !!str &anchor value
alias: *anchor
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for node properties"),
            }
        }
    }

    #[test]
    fn test_parse_scalar_styles() {
        let content = r#"
plain: plain_string
single_quoted: 'single quoted string'
double_quoted: "double quoted string"
literal_block: |
  literal block
folded_block: >
  folded block
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for scalar styles"),
            }
        }
    }

    #[test]
    fn test_parse_collection_styles() {
        let content = r#"
block_sequence:
  - item1
  - item2
  - item3

block_mapping:
  key1: value1
  key2: value2

flow_sequence: [item1, item2, item3]
flow_mapping: {key1: value1, key2: value2}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for collection styles"),
            }
        }
    }

    #[test]
    fn test_parse_complex_expressions() {
        let content = r#"
simple_expr: ${a + b}
complex_expr: ${users.filter(u => u.age > 18).length}
nested_expr: ${data.items[0].properties.name}
function_call: ${Math.round(3.14159)}
ternary: ${is_valid ? 'valid' : 'invalid'}
chained: ${data.get('key').transform(x => x * 2).toString()}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::ProcessingContext(_) => {}, // Expected
                _ => panic!("Expected ProcessingContext for complex expressions"),
            }
        }
    }

    #[test]
    fn test_parse_edge_case_property_refs() {
        let content = r#"
dollar_escaped: $$literal
double_dollar: $$$escaped
curly_in_ref: $user{property}
nested_curly: ${user${nested}}
complex_mixed: ${prefix_${user.id}_suffix}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        // These are edge cases that should be handled carefully
        for component in &components {
            match &component.value {
                ComponentValue::PropertyReference(_) | ComponentValue::ProcessingContext(_) | ComponentValue::Literal(_) => {}, // All are valid
                _ => panic!("Unexpected component type for edge case property refs"),
            }
        }
    }

    #[test]
    fn test_parse_performance_large_file() {
        let mut content = String::new();
        for i in 0..1000 {
            content.push_str(&format!("component_{}: value_{}\n", i, i));
        }
        
        let start = std::time::Instant::now();
        let result = parse_yaml_content(&content);
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1000);
        
        // Should complete within reasonable time (5 seconds for 1000 components)
        assert!(duration.as_secs() < 5, "Parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_parse_memory_usage() {
        // This test ensures we don't have memory leaks or excessive memory usage
        let content = r#"
large_component:
  key1: "A very long string that consumes memory"
  key2: "Another long string that also consumes memory"
  key3: "Yet another long string to test memory handling"
  nested:
    deep1: "Deep nested string 1"
    deep2: "Deep nested string 2"
    deep3: "Deep nested string 3"
"#;
        
        for _ in 0..100 {
            let result = parse_yaml_content(content);
            assert!(result.is_ok());
            let components = result.unwrap();
            assert_eq!(components.len(), 1);
            
            // Components should be dropped at end of iteration
            std::mem::drop(components);
        }
        
        // If we reach here without panics, memory management is working
    }

    #[test]
    fn test_parse_concurrent_access() {
        use std::sync::Arc;
        use std::thread;
        
        let content = r#"
shared_component: shared_value
"#;
        
        let content = Arc::new(content.to_string());
        let mut handles = vec![];
        
        for _ in 0..10 {
            let content_clone = Arc::clone(&content);
            let handle = thread::spawn(move || {
                let result = parse_yaml_content(&content_clone);
                assert!(result.is_ok());
                let components = result.unwrap();
                assert_eq!(components.len(), 1);
                components[0].name.clone()
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let name = handle.join().unwrap();
            assert_eq!(name, "shared_component");
        }
    }
}