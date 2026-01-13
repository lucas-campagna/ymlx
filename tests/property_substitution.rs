use ymx::*;
use std::collections::HashMap;
mod fixtures;
use fixtures::*;

#[cfg(test)]
mod property_substitution_tests {
    use super::*;

    #[test]
    fn test_substitute_properties_simple_literal() {
        let value = ComponentValue::Literal("Hello World!".to_string());
        let context = create_test_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_ok());
        
        match result.unwrap() {
            ComponentValue::Literal(s) => assert_eq!(s, "Hello World!"),
            _ => panic!("Expected Literal component"),
        }
    }

    #[test]
    fn test_substitute_properties_simple_property_ref() {
        let value = ComponentValue::PropertyReference("name".to_string());
        let context = create_test_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_ok());
        
        match result.unwrap() {
            ComponentValue::Literal(s) => assert_eq!(s, "World"),
            _ => panic!("Expected Literal component"),
        }
    }

    #[test]
    fn test_substitute_properties_property_ref_not_found() {
        let value = ComponentValue::PropertyReference("nonexistent".to_string());
        let context = create_test_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property 'nonexistent' not found"));
    }

    #[test]
    fn test_substitute_properties_empty_context() {
        let value = ComponentValue::PropertyReference("name".to_string());
        let context = create_empty_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property 'name' not found"));
    }

    #[test]
    fn test_substitute_properties_processing_context() {
        let value = ComponentValue::ProcessingContext("name + age".to_string());
        let context = create_test_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_ok());
        
        match result.unwrap() {
            ComponentValue::ProcessingContext(s) => assert_eq!(s, "name + age"),
            _ => panic!("Expected ProcessingContext component"),
        }
    }

    #[test]
    fn test_substitute_properties_component_call() {
        let mut properties = HashMap::new();
        properties.insert("text".to_string(), ComponentValue::Literal("Click Me".to_string()));
        
        let value = ComponentValue::ComponentCall(ComponentCall {
            target: "base_button".to_string(),
            properties,
        });
        let context = create_test_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property substitution failed"));
    }

    #[test]
    fn test_substitute_properties_nested_property_refs() {
        let mut nested_props = HashMap::new();
        nested_props.insert("username".to_string(), ComponentValue::PropertyReference("name".to_string()));
        
        let value = ComponentValue::ComponentCall(ComponentCall {
            target: "user_card".to_string(),
            properties: nested_props,
        });
        let context = create_test_context();
        
        // Component calls don't support property substitution yet
        let result = substitute_properties(&value, &context);
        assert!(result.is_err());
    }

    #[test]
    fn test_substitute_properties_template() {
        let value = ComponentValue::Template("Hello {{name}}!".to_string());
        let context = create_test_context();
        
        let result = substitute_properties(&value, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Property substitution failed"));
    }

    #[test]
    fn test_substitute_in_string_no_substitution() {
        let template = "Hello World!";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_substitute_in_string_simple_property() {
        let template = "Hello $name!";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_substitute_in_string_multiple_properties() {
        let template = "$name is $age years old from $city";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World is 25 years old from New York");
    }

    #[test]
    fn test_substitute_in_string_property_not_found() {
        let template = "Hello $nonexistent!";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello $nonexistent!");
    }

    #[test]
    fn test_substitute_in_string_processing_context() {
        let template = "Result: ${1 + 2}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: ${1 + 2}");
    }

    #[test]
    fn test_substitute_in_string_mixed() {
        let template = "Hello $name! You are ${age + 1} years old. Count: $count";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World! You are ${age + 1} years old. Count: 42");
    }

    #[test]
    fn test_substitute_in_string_empty_string() {
        let template = "";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_substitute_in_string_no_dollar() {
        let template = "No substitutions here";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No substitutions here");
    }

    #[test]
    fn test_substitute_in_string_only_dollar() {
        let template = "$";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$");
    }

    #[test]
    fn test_substitute_in_string_dollar_at_end() {
        let template = "Ending with $";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Ending with $");
    }

    #[test]
    fn test_substitute_in_string_double_dollar() {
        let template = "Price: $$100";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Price: $$100");
    }

    #[test]
    fn test_substitute_in_string_dollar_space() {
        let template = "$ name";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$ name");
    }

    #[test]
    fn test_substitute_in_string_property_with_underscore() {
        let template = "Value: $user_name";
        let mut context = create_test_context();
        context.insert("user_name".to_string(), "JohnDoe".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: JohnDoe");
    }

    #[test]
    fn test_substitute_in_string_property_with_dot() {
        let template = "Value: $user.name";
        let mut context = create_test_context();
        context.insert("user.name".to_string(), "John Doe".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: John Doe");
    }

    #[test]
    fn test_substitute_in_string_property_with_dash() {
        let template = "Value: $user-id";
        let mut context = create_test_context();
        context.insert("user-id".to_string(), "12345".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: 12345");
    }

    #[test]
    fn test_substitute_in_string_numeric_property() {
        let template = "Count: $count";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Count: 42");
    }

    #[test]
    fn test_substitute_in_string_boolean_property() {
        let template = "Active: $active";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Active: true");
    }

    #[test]
    fn test_substitute_in_string_empty_property_value() {
        let template = "Empty: $empty";
        let mut context = create_test_context();
        context.insert("empty".to_string(), "".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Empty: ");
    }

    #[test]
    fn test_substitute_in_string_property_with_spaces() {
        let template = "Value: $property with spaces";
        let mut context = create_test_context();
        context.insert("property with spaces".to_string(), "spaced value".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: spaced value");
    }

    #[test]
    fn test_substitute_in_string_case_sensitive() {
        let template = "Upper: $Name, Lower: $name";
        let mut context = create_test_context();
        context.insert("Name".to_string(), "UpperCase".to_string());
        context.insert("name".to_string(), "LowerCase".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Upper: UpperCase, Lower: LowerCase");
    }

    #[test]
    fn test_substitute_in_string_unicode_property() {
        let template = "Greeting: $greeting";
        let mut context = create_test_context();
        context.insert("greeting".to_string(), "你好世界".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Greeting: 你好世界");
    }

    #[test]
    fn test_substitute_in_string_emoji_property() {
        let template = "Emoji: $emoji";
        let mut context = create_test_context();
        context.insert("emoji".to_string(), "🚀".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Emoji: 🚀");
    }

    #[test]
    fn test_substitute_in_string_special_chars_property() {
        let template = "Special: $special";
        let mut context = create_test_context();
        context.insert("special".to_string(), "!@#$%^&*()".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Special: !@#$%^&*()");
    }

    #[test]
    fn test_substitute_in_string_very_long_property() {
        let long_prop = "p".repeat(1000);
        let template = format!("Long: ${}", long_prop);
        let mut context = create_test_context();
        context.insert(long_prop.clone(), "found".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Long: found");
    }

    #[test]
    fn test_substitute_in_string_very_long_property_value() {
        let long_value = "v".repeat(10000);
        let template = "Long: $long_prop";
        let mut context = create_test_context();
        context.insert("long_prop".to_string(), long_value.clone());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("Long: {}", long_value));
    }

    #[test]
    fn test_substitute_in_string_property_with_newlines() {
        let template = "Multiline: $multiline";
        let mut context = create_test_context();
        context.insert("multiline".to_string(), "Line 1\nLine 2\nLine 3".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\n'));
        assert_eq!(result.unwrap().lines().count(), 3);
    }

    #[test]
    fn test_substitute_in_string_property_with_tabs() {
        let template = "Tabbed: $tabbed";
        let mut context = create_test_context();
        context.insert("tabbed".to_string(), "Col1\tCol2\tCol3".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().contains('\t'));
    }

    #[test]
    fn test_substitute_in_string_nested_braces() {
        let template = "Nested: ${outer ${inner}}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        // Should preserve nested braces in processing context
        assert!(result.unwrap().contains("outer ${inner}"));
    }

    #[test]
    fn test_substitute_in_string_malformed_braces() {
        let template = "Malformed: ${unclosed";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated processing context"));
    }

    #[test]
    fn test_substitute_in_string_consecutive_dollars() {
        let template = "$$name$$$age";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$$name$$$age");
    }

    #[test]
    fn test_substitute_in_string_property_ref_in_processing_context() {
        let template = "Result: ${name + age}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: ${name + age}");
    }

    #[test]
    fn test_substitute_in_string_complex_processing_context() {
        let template = "Math: ${users.filter(u => u.age > 18).length}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Math: ${users.filter(u => u.age > 18).length}");
    }

    #[test]
    fn test_substitute_in_string_string_operations() {
        let template = "Concat: ${\"Hello, \" + name}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Concat: ${\"Hello, \" + name}");
    }

    #[test]
    fn test_substitute_in_string_ternary_operator() {
        let template = "Status: ${active ? \"Online\" : \"Offline\"}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Status: ${active ? \"Online\" : \"Offline\"}");
    }

    #[test]
    fn test_substitute_in_string_function_calls() {
        let template = "Result: ${Math.round(3.14159)}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: ${Math.round(3.14159)}");
    }

    #[test]
    fn test_substitute_in_string_array_access() {
        let template = "First: $users[0].name";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "First: $users[0].name");
    }

    #[test]
    fn test_substitute_in_string_method_chaining() {
        let template = "Length: ${data.get('items').length}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Length: ${data.get('items').length}");
    }

    #[test]
    fn test_substitute_in_string_regex_patterns() {
        let template = "Regex: ${text.replace(/\\d+/g, 'X')}";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Regex: ${text.replace(/\\d+/g, 'X')}");
    }

    #[test]
    fn test_substitute_in_string_escaped_dollar() {
        let template = "Escaped: \\$name";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Escaped: \\$name");
    }

    #[test]
    fn test_substitute_in_string_property_at_start() {
        let template = "$name at start";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World at start");
    }

    #[test]
    fn test_substitute_in_string_property_at_end() {
        let template = "at end $name";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "at end World");
    }

    #[test]
    fn test_substitute_in_string_property_only() {
        let template = "$name";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World");
    }

    #[test]
    fn test_substitute_in_string_multiple_same_property() {
        let template = "$name and $name";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World and World");
    }

    #[test]
    fn test_substitute_in_string_adjacent_properties() {
        let template = "$name$age";
        let context = create_test_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "World25");
    }

    #[test]
    fn test_substitute_in_string_performance() {
        let template = "Hello $name! You are $age years old from $city. Active: $active. Count: $count.";
        let context = create_test_context();
        
        let (_, duration) = measure_time(|| {
            for _ in 0..1000 {
                let result = substitute_in_string(template, &context);
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), "Hello World! You are 25 years old from New York. Active: true. Count: 42.");
            }
        });
        
        // Should complete within reasonable time (1 second for 1000 iterations)
        assert!(duration.as_secs() < 1, "Substitution took too long: {:?}", duration);
    }

    #[test]
    fn test_substitute_in_string_large_context() {
        let template = "Value: $key_500";
        let context = create_large_context();
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: value_500");
    }

    #[test]
    fn test_substitute_in_string_empty_property_name() {
        let template = "Value: $";
        let mut context = create_test_context();
        context.insert("".to_string(), "empty".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: $");
    }

    #[test]
    fn test_substitute_in_string_property_with_dollar() {
        let template = "Value: $dollar$prop";
        let mut context = create_test_context();
        context.insert("prop".to_string(), "value".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: $dollar$value");
    }

    #[test]
    fn test_substitute_in_string_mixed_case_property() {
        let template = "Mixed: $NaMe";
        let mut context = create_test_context();
        context.insert("NaMe".to_string(), "MixedCase".to_string());
        context.insert("name".to_string(), "lowercase".to_string());
        
        let result = substitute_in_string(template, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Mixed: MixedCase");
    }
}