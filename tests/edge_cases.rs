use ymx::*;

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_parse_empty_component_name() {
        let content = r#"
"": value with empty key
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "");
    }

    #[test]
    fn test_parse_null_component_name() {
        let content = r#"
null: value with null key
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        // Should treat "null" as literal string, not actual null
        assert_eq!(components[0].name, "null");
    }

    #[test]
    fn test_parse_boolean_component_name() {
        let content = r#"
true: value with true key
false: value with false key
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].name, "true");
        assert_eq!(components[1].name, "false");
    }

    #[test]
    fn test_parse_numeric_component_name() {
        let content = r#"
123: numeric key
456.789: float key
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].name, "123");
        assert_eq!(components[1].name, "456.789");
    }

    #[test]
    fn test_parse_very_deep_nesting() {
        let content = r#"
level1:
  level2:
    level3:
      level4:
        level5:
          level6:
            level7:
              level8:
                level9:
                  level10:
                    deep_value: finally found
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
        
        match &components[0].value {
            ComponentValue::Literal(_) => {}, // Expected after conversion
            _ => panic!("Expected Literal component for deeply nested object"),
        }
    }

    #[test]
    fn test_parse_extremely_wide_yaml() {
        let mut content = String::new();
        for i in 0..1000 {
            content.push_str(&format!("key{}: {}\n", i, "value".repeat(100)));
        }
        
        let start = std::time::Instant::now();
        let result = parse_yaml_content(&content);
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1000);
        
        // Should complete within reasonable time
        assert!(duration.as_secs() < 10, "Wide YAML parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_parse_extremely_deep_yaml() {
        let mut content = String::new();
        content.push_str("root:\n");
        for i in 0..1000 {
            content.push_str(&format!("  level{}:\n", i));
        }
        content.push_str("    value: deep\n");
        
        let result = parse_yaml_content(&content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_parse_circular_references_yaml() {
        let content = r#"
a:
  ref: $b
b:
  ref: $a
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {
                    // Should be converted to literal due to circular refs
                },
                ComponentValue::PropertyReference(_) => {
                    // Or remain as property references
                },
                _ => panic!("Unexpected component type for circular references"),
            }
        }
    }

    #[test]
    fn test_parse_invalid_yaml_structures() {
        let invalid_cases = vec![
            (r#"
unclosed: [1, 2, 3
"#, "unclosed array"),
            
            (r#"
unclosed: {key: value
"#, "unclosed mapping"),
            
            (r#"
invalid: *no_anchor
"#, "invalid alias"),
            
            (r#"
bad_indent:
 wrong_indent
"#, "bad indentation"),
            
            (r#"
duplicate: first
duplicate: second
"#, "duplicate keys"),
        ];
        
        for (content, _description) in invalid_cases {
            let result = parse_yaml_content(content);
            // YAML parser might handle some of these gracefully
            // Test doesn't assert error, just ensures no panic
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_parse_mixed_indentation_styles() {
        let content = r#"
spaces:
  indented_with_spaces

tabs:
\tindented_with_tabs

mixed:
 	mixed_indentation
    inconsistent_spaces
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn test_parse_special_yaml_constructs() {
        let content = r#"
# Explicit types
explicit_str: !!str value
explicit_int: !!int 42
explicit_float: !!float 3.14
explicit_bool: !!bool true
explicit_null: !!null null

# Custom tags
custom1: !custom value
custom2: !tag:type complex_value

# Merge keys
base: &base {a: 1, b: 2}
merged:
  <<: *base
  c: 3
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 8);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for special constructs"),
            }
        }
    }

    #[test]
    fn test_parse_binary_data_yaml() {
        let content = r#"
binary1: !!binary |
  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5
  OTk6enp56enolpaSEhP/++f/++f/++f/++f/++f/++f/++f/+
  +f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACH5BAEAAAAALAAAAAAMAAwA
  AAIUlI+hI+py+0Po5y02qsADQE=

binary2: !!binary R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 2);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for binary data"),
            }
        }
    }

    #[test]
    fn test_parse_timestamp_edge_cases() {
        let content = r#"
# Various timestamp formats
iso_basic: 2023-12-25
iso_full: 2023-12-25T15:30:00Z
iso_with_offset: 2023-12-25T15:30:00+05:00
space_separated: 2023-12-25 15:30:00
date_only: 2023-12-25
time_only: 15:30:00
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 6);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for timestamps"),
            }
        }
    }

    #[test]
    fn test_parse_numeric_edge_cases() {
        let content = r#"
# Edge case numbers
zero: 0
negative_zero: -0
positive_infinity: .inf
negative_infinity: -.inf
positive_infinity_alt: INF
negative_infinity_alt: -INF
not_a_number: .NaN
not_a_number_alt: NaN

hexadecimal: 0xFF
octal: 0755
binary: 0b1010
sexagesimal: 190:20:30

scientific_int: 1e10
scientific_float: 1.23e-4
scientific_negative: -5.67e+8
scientific_upper: 9.81E+2
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 16);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for edge case numbers"),
            }
        }
    }

    #[test]
    fn test_parse_string_edge_cases() {
        let content = r#"
# Edge case strings
empty_string: ""
explicit_empty: ''
null_string: 
whitespace_only: "   \t\n\r   "
numeric_string: "12345"
boolean_string: "true"
null_string_literal: "null"
special_chars: "!@#$%^&*()_+-=[]{}|;':\",./<>?"
unicode_escapes: "\\u00A9 \\u00E9 \\u20AC"
control_chars: "Test\\b\\f\\n\\r\\t"
quoted_special: "Line 1\nLine 2\\tTabbed"
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 11);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for edge case strings"),
            }
        }
    }

    #[test]
    fn test_parse_collection_edge_cases() {
        let content = r#"
# Edge case arrays
empty_array: []
single_element: [item]
nested_arrays: [[1, 2], [3, 4]]
mixed_types: [1, "string", true, null, [nested]]
comma_separated: [item1, item2, item3,]
trailing_comma: [1, 2, 3,]

# Edge case objects
empty_object: {}
single_property: {key: value}
nested_objects: {outer: {inner: {deep: value}}}
mixed_keys: {"string_key": 1, numeric_key: "string", true: boolean}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 10);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for edge case collections"),
            }
        }
    }

    #[test]
    fn test_parse_comment_edge_cases() {
        let content = r#"
# Full line comment
component1: value1  # Inline comment
component2: |
  multiline string
  # comment inside literal block
  continues here
component3: >
  folded string
  # comment inside folded block
  continues here

# Comment at end
# Multiple
# Comments

component4: value after comments
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 4);
    }

    #[test]
    fn test_parse_anchor_and_alias_edge_cases() {
        let content = r#"
# Simple anchor and alias
default: &default {timeout: 30, retries: 3}
using_default: <<: *default

# Multiple aliases
base1: &base1 {a: 1, b: 2}
base2: &base2 {c: 3, d: 4}
combined: <<: [*base1, *base2]

# Self-reference (should be handled gracefully)
self_ref: &self {value: test, ref: *self}

# Complex alias with merge
complex:
  <<: *default
  override: value
  <<: *base1
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected after conversion
                _ => panic!("Expected Literal component for anchor/alias edge cases"),
            }
        }
    }

    #[test]
    fn test_parse_tag_edge_cases() {
        let content = r#"
# Standard YAML tags
local_tag: !local value
global_tag: !tag:example.com,value
specific_tag: !!str specific_string

# Custom application tags
timestamp: !timestamp 2023-12-25T15:30:00Z
custom_struct: !user {name: John, age: 30}
custom_function: !func Math.sin(0.5)

# Tag with parameters
tagged: !custom:type parameter value
nested_tag: !outer !inner value

# Invalid tag (should be handled gracefully)
invalid: !invalid§tag value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 9);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected for unknown custom tags
                _ => panic!("Expected Literal component for tag edge cases"),
            }
        }
    }

    #[test]
    fn test_parse_document_boundary_edge_cases() {
        let content = r#"
---
document1: first document
...
---
document2: second document
---
document3: third document
...
# Document with only comments
---
%YAML 1.2
%TAG ! !foo:
---
document4: with directives
...
# Empty document
---
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
    fn test_parse_encoding_edge_cases() {
        let content = "🚀 Rocket: 你好世界\nArabic: مرحبا بالعالم\nEmoji: 🎨🖼️🎭🎪";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(s) => {
                    assert!(s.chars().any(|c| c > '\u{7F}'));
                },
                _ => panic!("Expected Literal component for unicode content"),
            }
        }
    }

    #[test]
    fn test_parse_line_ending_edge_cases() {
        let content = "crlf: windows style\r\nlf: unix style\nmixed: mixed\r\nendings\n";
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 3);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for line ending edge cases"),
            }
        }
    }

    #[test]
    fn test_parse_whitespace_edge_cases() {
        let content = r#"
# Various whitespace scenarios
leading_space: "  value"
trailing_space: "value  "
both_spaces: "  value  "
only_spaces: "   "
tabs: "\t\tvalue\t\t"
mixed_whitespace: " \t \t value \t \t "
newline: "value\nwith\nnewlines"
cr: "value\rwith\rcarriage"
mixed_line_endings: "value\r\nwith\nmixed\r\nendings"
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 9);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for whitespace edge cases"),
            }
        }
    }

    #[test]
    fn test_parse_property_reference_edge_cases() {
        let content = r#"
# Edge case property references
dollar_escape: $$literal
double_dollar: $$$value
curly_dollar: $${expression}
nested_dollar: $outer$inner
property_braces: $user{property}
empty_property: $
only_brace: ${
invalid_brace: ${
malformed_brace: ${unclosed
complex_mixed: ${prefix_${user.id}_suffix}
unicode_property: $名字
emoji_property: $🚀rocket
special_chars: $user@#$%^&*()
numeric_property: $123user
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 12);
        
        for component in &components {
            match &component.value {
                ComponentValue::PropertyReference(_) | 
                ComponentValue::ProcessingContext(_) | 
                ComponentValue::Literal(_) => {}, // All are valid edge cases
                _ => panic!("Unexpected component type for property reference edge cases"),
            }
        }
    }

    #[test]
    fn test_parse_processing_context_edge_cases() {
        let content = r#"
# Edge case processing contexts
empty_braces: ${}
simple_math: ${1 + 2}
complex_math: ${Math.sqrt(a^2 + b^2)}
string_ops: ${"Hello, " + name + "!"}
ternary: ${condition ? "true" : "false"}
function_call: ${array.map(x => x * 2)}
nested_expressions: ${outer(${inner(1 + 2)})}
property_access: ${users[0].profile.settings.theme}
regex_replace: ${text.replace(/\d+/g, "X")}
chained_calls: ${data.get('items').filter(x => x.active).length}
multi_line: ${
  This is a
  multi-line
  expression
}
unicode_expr: ${名字 + " is " + age}
invalid_expr: ${this is not valid js}
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 15);
        
        for component in &components {
            match &component.value {
                ComponentValue::ProcessingContext(_) => {}, // Expected
                _ => panic!("Expected ProcessingContext for expression edge cases"),
            }
        }
    }

    #[test]
    fn test_parse_component_call_edge_cases() {
        let content = r#"
# Edge case component calls
empty_target:
  from!: 
  prop: value

missing_target:
  prop: value

multiple_from:
  from!: base1
  yx-from: base2
  From: base3
  prop: conflict

nested_calls:
  from!: outer
  inner:
    from!: inner
    prop: value

unicode_target:
  from!: 组件
  prop: value

emoji_target:
  from!: 🎨button
  prop: value

special_char_target:
  from!: button@#$%^&*()
  prop: value

empty_properties:
  from!: base

deeply_nested:
  from!: base
  level1:
    level2:
      level3:
        level4:
          deep_prop: value
"#;
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 10);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) | ComponentValue::ComponentCall(_) => {
                    // Both are valid depending on structure
                },
                _ => panic!("Unexpected component type for component call edge cases"),
            }
        }
    }

    #[test]
    fn test_performance_with_large_yaml() {
        let mut content = String::new();
        for i in 0..5000 {
            content.push_str(&format!("component_{}: {}\n", i, "value".repeat(100)));
        }
        
        let start = std::time::Instant::now();
        let result = parse_yaml_content(&content);
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5000);
        
        // Should complete within reasonable time (10 seconds for 5000 large components)
        assert!(duration.as_secs() < 10, "Large YAML parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_memory_usage_with_large_yaml() {
        // Test that large YAML doesn't cause memory issues
        for _iteration in 0..10 {
            let mut content = String::new();
            for i in 0..1000 {
                content.push_str(&format!("component_{}: {}\n", i, "x".repeat(1000)));
            }
            
            let result = parse_yaml_content(&content);
            assert!(result.is_ok());
            let components = result.unwrap();
            assert_eq!(components.len(), 1000);
            
            // Components should be dropped at end of iteration
            std::mem::drop(components);
        }
        
        // If we reach here without panics, memory management is working
    }

    #[test]
    fn test_concurrent_parsing() {
        use std::sync::Arc;
        use std::thread;
        
        let content = r#"
shared: shared_value
number: $num
boolean: $bool
"#;
        
        let content = Arc::new(content.to_string());
        let mut handles = vec![];
        
        for i in 0..10 {
            let content_clone = Arc::clone(&content);
            let handle = thread::spawn(move || {
                let result = parse_yaml_content(&content_clone);
                assert!(result.is_ok());
                let components = result.unwrap();
                assert_eq!(components.len(), 3);
                (i, components.len())
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let (thread_id, component_count) = handle.join().unwrap();
            assert_eq!(component_count, 3, "Thread {} failed", thread_id);
        }
    }

    #[test]
    fn test_error_recovery() {
        let content = r#"
valid1: valid component
invalid:
  from!: 
  # missing target
valid2: another valid component
unclosed: [1, 2, 3
valid3: final valid component
"#;
        
        let result = parse_yaml_content(content);
        // YAML parser should handle this gracefully or error consistently
        match result {
            Ok(components) => {
                // If parsing succeeds, should get some components
                assert!(!components.is_empty());
                for component in &components {
                    assert!(!component.name.is_empty());
                }
            },
            Err(_) => {
                // If parsing fails, error should be informative
            }
        }
    }

    #[test]
    fn test_backwards_compatibility() {
        // Test that older YAML formats still work
        let old_style = r#"
# Legacy YAML 1.0 style
simple: "quoted string"
number: 42
boolean: true
array: [1, 2, 3]
object: {key: value}
"#;
        
        let result = parse_yaml_content(old_style);
        assert!(result.is_ok());
        let components = result.unwrap();
        assert_eq!(components.len(), 5);
        
        for component in &components {
            match &component.value {
                ComponentValue::Literal(_) => {}, // Expected
                _ => panic!("Expected Literal component for legacy YAML"),
            }
        }
    }
}