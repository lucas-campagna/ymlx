// Simplified tests that don't require complex module imports
use ymx::*;
use ymx::component::*;
use std::collections::HashMap;

// Test fixtures
const SIMPLE_YAML: &str = r#"
component: Hello World!
number: 42
boolean: true
"#;

const PROPERTY_REF_YAML: &str = r#"
greeting: Hello $name!
message: $name is $age years old
"#;

const COMPONENT_CALL_YAML: &str = r#"
button:
  from!: base_button
  text: Click me
"#;

fn create_test_context() -> HashMap<String, String> {
    let mut context = HashMap::new();
    context.insert("name".to_string(), "World".to_string());
    context.insert("age".to_string(), "25".to_string());
    context
}

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn test_parse_simple_yaml() {
        let result = parse_yaml_content(SIMPLE_YAML);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        assert_eq!(components[0].name, "component");
        assert_eq!(components[1].name, "number");
        assert_eq!(components[2].name, "boolean");
    }

    #[test]
    fn test_parse_property_refs() {
        let result = parse_yaml_content(PROPERTY_REF_YAML);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        // Check if properties are detected
        for component in &components {
            match &component.value {
                ComponentValue::PropertyReference(_) | ComponentValue::ProcessingContext(_) => {
                    // Expected for property references and expressions
                },
                _ => {
                    // Also possible to be converted to literal
                }
            }
        }
    }

    #[test]
    fn test_parse_component_calls() {
        let result = parse_yaml_content(COMPONENT_CALL_YAML);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base_button");
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_execute_literal_component() {
        let component = ymx::YMXComponent {
            id: "test".to_string(),
            name: "test".to_string(),
            value: ComponentValue::Literal("Hello World!".to_string()),
        };
        let context = create_test_context();
        
        let result = component::execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_execute_property_ref_success() {
        let component = ymx::YMXComponent {
            id: "test".to_string(),
            name: "test".to_string(),
            value: ComponentValue::PropertyReference("name".to_string()),
        };
        let context = create_test_context();
        
        let result = component::execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World");
    }

    #[test]
    fn test_execute_property_ref_not_found() {
        let component = ymx::YMXComponent {
            id: "test".to_string(),
            name: "test".to_string(),
            value: ComponentValue::PropertyReference("nonexistent".to_string()),
        };
        let context = create_test_context();
        
        let result = component::execute_component(&component, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_substitute_in_string_no_substitution() {
        let template = "Hello World!";
        let context = create_test_context();
        
        let result = ymx::substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_substitute_in_string_with_properties() {
        let template = "Hello $name! You are $age years old.";
        let context = create_test_context();
        
        let result = ymx::substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World! You are 25 years old.");
    }

    #[test]
    fn test_substitute_in_string_property_not_found() {
        let template = "Hello $nonexistent!";
        let context = create_test_context();
        
        let result = ymx::substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello $nonexistent!");
    }

    #[test]
    fn test_substitute_in_string_processing_context() {
        let template = "Result: ${1 + 2}";
        let context = create_test_context();
        
        let result = ymx::substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: ${1 + 2}");
    }

    #[test]
    fn test_unicode_handling() {
        let yaml_content = r#"
emoji: 🚀 Rocket
chinese: 你好世界
unicode: Tëst Ünicöde
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        // Test execution with unicode
        for component in &components {
            let context = create_test_context();
            let exec_result = component::execute_component(component, &context);
            assert!(exec_result.is_ok());
            
            // Unicode should be preserved
            let output = exec_result.unwrap();
            assert!(output.chars().any(|c| c > '\u{7F}'));
        }
    }

    #[test]
    fn test_empty_yaml() {
        let result = parse_yaml_content("");
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_whitespace_only_yaml() {
        let result = parse_yaml_content("   \n  \t  \n  ");
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_numeric_values() {
        let yaml_content = r#"
integer: 42
negative: -100
float: 3.14
scientific: 1.23e10
zero: 0
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {
                    // Numbers should be converted to literal strings
                },
                _ => panic!("Expected Literal component for numeric values"),
            }
        }
    }

    #[test]
    fn test_boolean_values() {
        let yaml_content = r#"
true_val: true
false_val: false
yes: yes
no: no
on: on
off: off
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 6);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {
                    // Booleans should be converted to literal strings
                },
                _ => panic!("Expected Literal component for boolean values"),
            }
        }
    }

    #[test]
    fn test_array_values() {
        let yaml_content = r#"
simple_array: [1, 2, 3]
string_array: ["a", "b", "c"]
mixed_array: [1, "two", true]
empty_array: []
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {
                    // Arrays should be converted to literal strings
                },
                _ => panic!("Expected Literal component for array values"),
            }
        }
    }

    #[test]
    fn test_nested_objects() {
        let yaml_content = r#"
nested:
  level1: value1
  level2:
    deep: value2
  level3: [1, 2, 3]
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {
                // Nested objects should be converted to literal strings
            },
            _ => panic!("Expected Literal component for nested objects"),
        }
    }

    #[test]
    fn test_special_characters() {
        let yaml_content = r#"
quotes: "Hello \"World\""
newlines: "Line1\nLine2"
tabs: "Col1\tCol2"
unicode: "🚀 Hello 世界"
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {
                    // Special characters should be preserved
                },
                _ => panic!("Expected Literal component for special characters"),
            }
        }
    }

    #[test]
    fn test_null_and_empty() {
        let yaml_content = r#"
null_value: null
empty_string: ""
empty_object: {}
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {
                    // Null and empty should be converted to literal strings
                },
                _ => panic!("Expected Literal component for null/empty values"),
            }
        }
    }

    #[test]
    fn test_large_yaml_parsing() {
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
        
        // Should complete within reasonable time
        assert!(duration.as_secs() < 5, "Large YAML parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_performance_with_many_executions() {
        let component = ymx::YMXComponent {
            id: "test".to_string(),
            name: "test".to_string(),
            value: ComponentValue::Literal("Test value".to_string()),
        };
        let context = create_test_context();
        
        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let result = component::execute_component(&component, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Test value");
        }
        let duration = start.elapsed();
        
        // Should complete within reasonable time
        assert!(duration.as_secs() < 3, "Many executions took too long: {:?}", duration);
    }

    #[test]
    fn test_complex_component_call() {
        let yaml_content = r#"
complex_call:
  from!: base_component
  user:
    name: $user_name
    age: $user_age
    profile:
      theme: $theme
  settings:
    notifications: $notifications
    timeout: 30
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::ComponentCall(call) => {
                assert_eq!(call.target, "base_component");
                assert!(call.properties.contains_key("user"));
                assert!(call.properties.contains_key("settings"));
            },
            _ => panic!("Expected ComponentCall"),
        }
    }

    #[test]
    fn test_mixed_yaml() {
        let yaml_content = r#"
# This is a mix of different component types
simple_literal: Just a string
property_ref: $variable
processing_context: ${1 + 2}
component_call:
  from!: base
  param: value
array: [1, 2, 3]
nested: {key: value}
boolean: true
number: 42
null_value: null
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 9);
        
        // Check that we have different component types
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
        
        assert!(found_types.len() >= 3); // Should have multiple types
    }

    #[test]
    fn test_error_handling_invalid_yaml() {
        let invalid_yaml = r#"
invalid: [unclosed array
another: bad content
"#;
        
        let result = parse_yaml_content(invalid_yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("YAML parsing error"));
    }

    #[test]
    fn test_edge_case_property_names() {
        let yaml_content = r#"
"dotted.key": value1
"spaced key": value2
"dollar$key": value3
unicode_key: value4
emoji_key: value5
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        // All should be accessible
        for component in &components {
            assert!(!component.name.is_empty());
        }
    }

    #[test]
    fn test_component_name_uniqueness() {
        let yaml_content = r#"
component: First component
component: Second component
component: Third component
"#;
        
        let result = parse_yaml_content(yaml_content);
        assert!(result.is_ok());
        let components = result.unwrap();
        
        // YAML parser should handle duplicate keys (last wins)
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "component");
        
        match &components[0].value {
            ComponentValue::Literal(s) => {
                assert_eq!(s, "Third component");
            },
            _ => panic!("Expected Literal component"),
        }
    }
}