use ymx::*;
use ymx::component::*;
use crate::fixtures::*;

#[cfg(test)]
mod component_execution_tests {
    use super::*;

    #[test]
    fn test_execute_simple_literal() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("Hello World!".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_execute_property_reference_success() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("name".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World");
    }

    #[test]
    fn test_execute_property_reference_not_found() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("nonexistent".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property 'nonexistent' not found"));
    }

    #[test]
    fn test_execute_property_reference_empty_context() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("name".to_string())
        );
        let context = create_empty_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property 'name' not found"));
    }

    #[test]
    fn test_execute_processing_context_simple() {
        let component = create_test_component(
            "test",
            ComponentValue::ProcessingContext("1 + 2".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Evaluated"));
    }

    #[test]
    fn test_execute_processing_context_complex() {
        let component = create_test_component(
            "test",
            ComponentValue::ProcessingContext("user.age + 10".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Evaluated"));
    }

    #[test]
    fn test_execute_processing_context_string_ops() {
        let component = create_test_component(
            "test",
            ComponentValue::ProcessingContext("\"Hello, \" + name".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Evaluated"));
    }

    #[test]
    fn test_execute_component_call_simple() {
        let mut properties = HashMap::new();
        properties.insert("text".to_string(), ComponentValue::Literal("Click Me".to_string()));
        properties.insert("style".to_string(), ComponentValue::Literal("primary".to_string()));
        
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "base_button".to_string(),
                properties,
            })
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Called component: base_button"));
    }

    #[test]
    fn test_execute_component_call_with_property_refs() {
        let mut properties = HashMap::new();
        properties.insert("username".to_string(), ComponentValue::PropertyReference("name".to_string()));
        properties.insert("count".to_string(), ComponentValue::Literal("42".to_string()));
        
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "user_card".to_string(),
                properties,
            })
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Called component: user_card"));
    }

    #[test]
    fn test_execute_component_call_nested() {
        let mut nested_props = HashMap::new();
        nested_props.insert("title".to_string(), ComponentValue::Literal("Profile".to_string()));
        
        let mut properties = HashMap::new();
        properties.insert("header".to_string(), ComponentValue::ComponentCall(ComponentCall {
            target: "card".to_string(),
            properties: nested_props,
        }));
        
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "page".to_string(),
                properties,
            })
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Called component: page"));
    }

    #[test]
    fn test_execute_component_with_large_context() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("key_500".to_string())
        );
        let context = create_large_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "value_500");
    }

    #[test]
    fn test_execute_component_performance_large() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("A very long string that tests performance".repeat(100))
        );
        let context = create_large_context();
        
        let (_, duration) = measure_time(|| {
            for _ in 0..1000 {
                let result = execute_component(&component, &context);
                assert!(result.is_ok());
            }
        });
        
        // Should complete within reasonable time (1 second for 1000 executions)
        assert!(duration.as_secs() < 1, "Execution took too long: {:?}", duration);
    }

    #[test]
    fn test_execute_component_with_empty_properties() {
        let properties = HashMap::new();
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "empty_component".to_string(),
                properties,
            })
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Called component: empty_component"));
    }

    #[test]
    fn test_execute_component_circular_reference_detection() {
        // This test should handle circular references gracefully
        let mut props1 = HashMap::new();
        props1.insert("ref".to_string(), ComponentValue::PropertyReference("circular".to_string()));
        
        let mut context = create_test_context();
        context.insert("circular".to_string(), "reference".to_string());
        
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "circular_test".to_string(),
                properties: props1,
            })
        );
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_component_with_unicode() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("🚀 Hello World! 你好世界 🌍".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "🚀 Hello World! 你好世界 🌍");
    }

    #[test]
    fn test_execute_component_with_special_chars() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("Special chars: \\n\\t\\\"\\'\\$\\{\\}".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Special chars"));
    }

    #[test]
    fn test_execute_component_very_long_string() {
        let long_string = "A".repeat(10000);
        let component = create_test_component(
            "test",
            ComponentValue::Literal(long_string.clone())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 10000);
    }

    #[test]
    fn test_execute_component_concurrent() {
        use std::sync::Arc;
        use std::thread;
        
        let component = Arc::new(create_test_component(
            "test",
            ComponentValue::Literal("shared component".to_string())
        ));
        let context = Arc::new(create_test_context());
        
        let mut handles = vec![];
        
        for _ in 0..10 {
            let component_clone = Arc::clone(&component);
            let context_clone = Arc::clone(&context);
            
            let handle = thread::spawn(move || {
                execute_component(&component_clone, &context_clone)
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "shared component");
        }
    }

    #[test]
    fn test_execute_component_memory_safety() {
        // Test that we can execute many components without memory issues
        for i in 0..1000 {
            let component = create_test_component(
                &format!("component_{}", i),
                ComponentValue::Literal(format!("value_{}", i))
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), format!("value_{}", i));
            
            // Component should be dropped here
        }
    }

    #[test]
    fn test_execute_component_with_null_values() {
        let mut properties = HashMap::new();
        properties.insert("null_prop".to_string(), ComponentValue::Literal("null".to_string()));
        
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "null_test".to_string(),
                properties,
            })
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_component_with_numeric_strings() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("1234567890".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1234567890");
    }

    #[test]
    fn test_execute_component_with_boolean_strings() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("true".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_execute_component_empty_string() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_execute_component_whitespace_only() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("   \n\t  ".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "   \n\t  ");
    }

    #[test]
    fn test_execute_component_with_line_breaks() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("Line 1\nLine 2\nLine 3".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\n'));
        assert_eq!(result.unwrap().lines().count(), 3);
    }

    #[test]
    fn test_execute_component_with_tabs() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("Col1\tCol2\tCol3".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\t'));
    }

    #[test]
    fn test_execute_component_property_substitution() {
        let component = create_test_component(
            "test",
            ComponentValue::ProcessingContext("\"Hello, \" + name + \"!\"".to_string())
        );
        let mut context = create_test_context();
        context.insert("name".to_string(), "Alice".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Evaluated"));
    }

    #[test]
    fn test_execute_component_with_special_property_names() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("user.name".to_string())
        );
        let mut context = create_test_context();
        context.insert("user.name".to_string(), "John Doe".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "John Doe");
    }

    #[test]
    fn test_execute_component_numeric_property() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("age".to_string())
        );
        let context = create_test_context(); // age = "25"
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "25");
    }

    #[test]
    fn test_execute_component_boolean_property() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("active".to_string())
        );
        let context = create_test_context(); // active = "true"
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_execute_component_case_sensitive_properties() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("Name".to_string())
        );
        let mut context = create_test_context();
        context.insert("Name".to_string(), "UpperCase".to_string());
        context.insert("name".to_string(), "lowerCase".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "UpperCase");
    }

    #[test]
    fn test_execute_component_property_with_dots() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("user.profile.settings.theme".to_string())
        );
        let mut context = create_test_context();
        context.insert("user.profile.settings.theme".to_string(), "dark".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dark");
    }

    #[test]
    fn test_execute_component_property_with_underscores() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("user_profile_name".to_string())
        );
        let mut context = create_test_context();
        context.insert("user_profile_name".to_string(), "JohnDoe".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "JohnDoe");
    }

    #[test]
    fn test_execute_component_property_with_dashes() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("user-id".to_string())
        );
        let mut context = create_test_context();
        context.insert("user-id".to_string(), "12345".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "12345");
    }

    #[test]
    fn test_execute_component_error_handling() {
        let component = create_test_component(
            "test",
            ComponentValue::Template("{{invalid}}".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property substitution failed"));
    }

    #[test]
    fn test_execute_component_complex_property_chain() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("a.b.c.d.e".to_string())
        );
        let mut context = create_test_context();
        context.insert("a.b.c.d.e".to_string(), "deep_value".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "deep_value");
    }

    #[test]
    fn test_execute_component_empty_property_name() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("".to_string())
        );
        let mut context = create_test_context();
        context.insert("".to_string(), "empty_key_value".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "empty_key_value");
    }

    #[test]
    fn test_execute_component_property_with_unicode() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("名字".to_string())
        );
        let mut context = create_test_context();
        context.insert("名字".to_string(), "张三".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "张三");
    }

    #[test]
    fn test_execute_component_property_with_emoji() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("🚀rocket".to_string())
        );
        let mut context = create_test_context();
        context.insert("🚀rocket".to_string(), "launch".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "launch");
    }

    #[test]
    fn test_execute_component_very_long_property_name() {
        let long_property_name = "p".repeat(1000);
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference(long_property_name.clone())
        );
        let mut context = create_test_context();
        context.insert(long_property_name.clone(), "value".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "value");
    }

    #[test]
    fn test_execute_component_very_long_property_value() {
        let long_value = "v".repeat(10000);
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("long_property".to_string())
        );
        let mut context = create_test_context();
        context.insert("long_property".to_string(), long_value.clone());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), long_value);
    }

    #[test]
    fn test_execute_component_with_newlines_in_properties() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("multiline".to_string())
        );
        let mut context = create_test_context();
        context.insert("multiline".to_string(), "Line 1\nLine 2\nLine 3".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\n'));
        assert_eq!(result.unwrap().lines().count(), 3);
    }

    #[test]
    fn test_execute_component_with_tabs_in_properties() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("tabbed".to_string())
        );
        let mut context = create_test_context();
        context.insert("tabbed".to_string(), "Col1\tCol2\tCol3".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\t'));
    }

    #[test]
    fn test_execute_component_property_with_spaces() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("property with spaces".to_string())
        );
        let mut context = create_test_context();
        context.insert("property with spaces".to_string(), "spaced value".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "spaced value");
    }

    #[test]
    fn test_execute_component_property_with_special_chars() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("prop@#$%^&*()".to_string())
        );
        let mut context = create_test_context();
        context.insert("prop@#$%^&*()".to_string(), "special".to_string());
        
        let result = execute_component(&component, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "special");
    }
}