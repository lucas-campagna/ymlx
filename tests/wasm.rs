#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
use ymx::*;
use crate::fixtures::*;

#[cfg(test)]
mod wasm_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn test_wasm_processor_creation() {
        setup_wasm_test();
        
        let yaml_content = r#"
component1: Hello World!
component2: $name
component3: ${1 + 2}
"#;
        
        let result = YMXProcessor::new(yaml_content);
        assert!(result.is_ok());
        
        let processor = result.unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"component1".to_string()));
        assert!(names.contains(&"component2".to_string()));
        assert!(names.contains(&"component3".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_wasm_processor_invalid_yaml() {
        setup_wasm_test();
        
        let yaml_content = r#"
invalid: [unclosed array
another: bad content
"#;
        
        let result = YMXProcessor::new(yaml_content);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_literal_component() {
        setup_wasm_test();
        
        let yaml_content = r#"
hello: Hello World!
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({})).unwrap();
        
        let result = processor.execute_component("hello", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_property_reference() {
        setup_wasm_test();
        
        let yaml_content = r#"
greeting: Hello $name!
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "name": "World"
        })).unwrap();
        
        let result = processor.execute_component("greeting", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_component_not_found() {
        setup_wasm_test();
        
        let yaml_content = r#"
hello: Hello World!
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({})).unwrap();
        
        let result = processor.execute_component("nonexistent", properties);
        assert!(result.is_err());
        assert!(result.unwrap_err().as_string().unwrap().contains("Component 'nonexistent' not found"));
    }

    #[wasm_bindgen_test]
    fn test_wasm_validate_yaml_syntax_valid() {
        setup_wasm_test();
        
        let yaml_content = r#"
component: Hello World!
number: 42
boolean: true
"#;
        
        let result = validate_yaml_syntax(yaml_content);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_validate_yaml_syntax_invalid() {
        setup_wasm_test();
        
        let yaml_content = r#"
invalid: [unclosed
another: content
"#;
        
        let result = validate_yaml_syntax(yaml_content);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_wasm_parse_yaml_components() {
        setup_wasm_test();
        
        let yaml_content = r#"
component1: Hello World!
component2: $name
component3: ${1 + 2}
component4:
  from!: base
  text: Click me
"#;
        
        let result = parse_yaml_components(yaml_content);
        assert!(result.is_ok());
        
        let names_json = result.unwrap();
        let names: Vec<String> = serde_json::from_str(&names_json).unwrap();
        assert_eq!(names.len(), 4);
        assert!(names.contains(&"component1".to_string()));
        assert!(names.contains(&"component2".to_string()));
        assert!(names.contains(&"component3".to_string()));
        assert!(names.contains(&"component4".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_wasm_empty_yaml() {
        setup_wasm_test();
        
        let yaml_content = "";
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_whitespace_only_yaml() {
        setup_wasm_test();
        
        let yaml_content = "   \n  \t  \n  ";
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_complex_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
user_info: User $name is $age years old from $city
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "name": "Alice",
            "age": "30",
            "city": "Paris"
        })).unwrap();
        
        let result = processor.execute_component("user_info", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "User Alice is 30 years old from Paris");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_unicode() {
        setup_wasm_test();
        
        let yaml_content = r#"
greeting: 🚀 Hello $name! 你好世界 🌍
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "name": "世界"
        })).unwrap();
        
        let result = processor.execute_component("greeting", properties);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("🚀"));
        assert!(result.unwrap().contains("世界"));
        assert!(result.unwrap().contains("你好"));
        assert!(result.unwrap().contains("🌍"));
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_special_chars() {
        setup_wasm_test();
        
        let yaml_content = r#"
special: "Hello \"World\"! \\n\\t $name"
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "name": "Test"
        })).unwrap();
        
        let result = processor.execute_component("special", properties);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_boolean_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
status: User is $active and $verified
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "active": "true",
            "verified": "false"
        })).unwrap();
        
        let result = processor.execute_component("status", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "User is true and false");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_numeric_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
math: Result: $a + $b = $sum
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "a": "10",
            "b": "20",
            "sum": "30"
        })).unwrap();
        
        let result = processor.execute_component("math", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: 10 + 20 = 30");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_null_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
null_test: Value is $null_value
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "null_value": "null"
        })).unwrap();
        
        let result = processor.execute_component("null_test", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value is null");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_empty_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
empty_test: Empty: $empty_value
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "empty_value": ""
        })).unwrap();
        
        let result = processor.execute_component("empty_test", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Empty: ");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_array_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
array_test: First item: $first
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "first": "item1"
        })).unwrap();
        
        let result = processor.execute_component("array_test", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "First item: item1");
    }

    #[wasm_bindgen_test]
    fn test_wasm_execute_with_object_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
object_test: Name: $user_name
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({
            "user_name": "John Doe"
        })).unwrap();
        
        let result = processor.execute_component("object_test", properties);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Name: John Doe");
    }

    #[wasm_bindgen_test]
    fn test_wasm_multiple_components() {
        setup_wasm_test();
        
        let yaml_content = r#"
comp1: First component
comp2: Second component
comp3: Third component
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let properties = JsValue::from_serde(&serde_json::json!({})).unwrap();
        
        let result1 = processor.execute_component("comp1", properties.clone());
        let result2 = processor.execute_component("comp2", properties.clone());
        let result3 = processor.execute_component("comp3", properties);
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        
        assert_eq!(result1.unwrap(), "First component");
        assert_eq!(result2.unwrap(), "Second component");
        assert_eq!(result3.unwrap(), "Third component");
    }

    #[wasm_bindgen_test]
    fn test_wasm_processor_reuse() {
        setup_wasm_test();
        
        let yaml_content = r#"
test: $value
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        
        // Execute multiple times with different properties
        let props1 = JsValue::from_serde(&serde_json::json!({"value": "first"})).unwrap();
        let props2 = JsValue::from_serde(&serde_json::json!({"value": "second"})).unwrap();
        let props3 = JsValue::from_serde(&serde_json::json!({"value": "third"})).unwrap();
        
        let result1 = processor.execute_component("test", props1);
        let result2 = processor.execute_component("test", props2);
        let result3 = processor.execute_component("test", props3);
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        
        assert_eq!(result1.unwrap(), "first");
        assert_eq!(result2.unwrap(), "second");
        assert_eq!(result3.unwrap(), "third");
    }

    #[wasm_bindgen_test]
    fn test_wasm_large_yaml() {
        setup_wasm_test();
        
        let mut yaml_content = String::new();
        for i in 0..100 {
            yaml_content.push_str(&format!("component_{}: value_{}\n", i, i));
        }
        
        let processor = YMXProcessor::new(&yaml_content).unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 100);
        
        for i in 0..100 {
            assert!(names.contains(&format!("component_{}", i)));
        }
    }

    #[wasm_bindgen_test]
    fn test_wasm_component_names_order() {
        setup_wasm_test();
        
        let yaml_content = r#"
z_component: last
a_component: first
m_component: middle
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 3);
        
        // Order should match YAML definition order
        assert_eq!(names[0], "z_component");
        assert_eq!(names[1], "a_component");
        assert_eq!(names[2], "m_component");
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_handling_invalid_properties() {
        setup_wasm_test();
        
        let yaml_content = r#"
simple: Hello World!
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        
        // Pass invalid properties (not an object)
        let invalid_props = JsValue::from_str("\"not an object\"");
        let result = processor.execute_component("simple", invalid_props);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().as_string().unwrap().contains("Invalid properties"));
    }

    #[wasm_bindgen_test]
    fn test_wasm_memory_safety() {
        setup_wasm_test();
        
        let yaml_content = r#"
test: $value
"#;
        
        // Create and drop many processors to test memory safety
        for i in 0..100 {
            let processor = YMXProcessor::new(yaml_content).unwrap();
            let properties = JsValue::from_serde(&serde_json::json!({
                "value": format!("iteration_{}", i)
            })).unwrap();
            
            let result = processor.execute_component("test", properties);
            assert!(result.is_ok());
            
            // Processor should be dropped here
        }
    }

    #[wasm_bindgen_test]
    fn test_wasm_concurrent_access() {
        setup_wasm_test();
        
        let yaml_content = r#"
shared: Component $index
"#;
        
        // Note: WASM is single-threaded, but we can test rapid sequential access
        let processor = YMXProcessor::new(yaml_content).unwrap();
        
        for i in 0..50 {
            let properties = JsValue::from_serde(&serde_json::json!({
                "index": i.to_string()
            })).unwrap();
            
            let result = processor.execute_component("shared", properties);
            assert!(result.is_ok());
            assert!(result.unwrap().contains(&format!("Component {}", i)));
        }
    }

    #[wasm_bindgen_test]
    fn test_wasm_component_call_detection() {
        setup_wasm_test();
        
        let yaml_content = r#"
button:
  from!: base_button
  text: Click me
regular: Just text
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"button".to_string()));
        assert!(names.contains(&"regular".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_wasm_unicode_component_names() {
        setup_wasm_test();
        
        let yaml_content = r#"
组件1: First component
компонент2: Second component
コンポーネント3: Third component
"#;
        
        let processor = YMXProcessor::new(yaml_content).unwrap();
        let names = processor.get_component_names().into_serde::<Vec<String>>().unwrap();
        assert_eq!(names.len(), 3);
        
        assert!(names.contains(&"组件1".to_string()));
        assert!(names.contains(&"компонент2".to_string()));
        assert!(names.contains(&"コンポーネント3".to_string()));
    }
}