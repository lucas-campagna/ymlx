// Test fixtures and utilities for YMX tests
use std::collections::HashMap;
use ymx::*;

pub fn create_test_component(name: &str, value: ComponentValue) -> YMXComponent {
    YMXComponent {
        id: name.to_string(),
        name: name.to_string(),
        value,
    }
}

pub fn create_test_context() -> HashMap<String, String> {
    let mut context = HashMap::new();
    context.insert("name".to_string(), "World".to_string());
    context.insert("age".to_string(), "25".to_string());
    context.insert("city".to_string(), "New York".to_string());
    context.insert("count".to_string(), "42".to_string());
    context.insert("active".to_string(), "true".to_string());
    context
}

pub fn create_empty_context() -> HashMap<String, String> {
    HashMap::new()
}

pub fn create_large_context() -> HashMap<String, String> {
    let mut context = HashMap::new();
    for i in 0..1000 {
        context.insert(format!("key_{}", i), format!("value_{}", i));
    }
    context
}

// Test YAML fixtures
pub const SIMPLE_YAML: &str = r#"
component: Hello World!
number: 42
boolean: true
"#;

pub const PROPERTY_REF_YAML: &str = r#"
greeting: Hello $name!
message: $name is $age years old
info: User $name from $city is active: $active
"#;

pub const PROCESSING_CONTEXT_YAML: &str = r#"
calculation: ${1 + 2}
complex_expr: ${user.age + 10}
string_expr: ${"Hello, " + name}
conditional: ${active ? "Online" : "Offline"}
"#;

pub const COMPONENT_CALL_YAML: &str = r#"
button:
  from!: base_button
  text: Click me
  style: primary

card:
  yx-from: base_card
  title: User Card
  content:
    name: $user.name
    email: $user.email

modal:
  From: base_modal
  title: Confirmation
  size: large
"#;

pub const COMPLEX_YAML: &str = r#"
# Simple literals
simple_string: Hello World!
simple_number: 42
simple_boolean: true

# Property references
prop_ref: $name
nested_ref: $user.profile.name
array_ref: $users[0].name

# Processing contexts
simple_expr: ${1 + 2}
complex_expr: ${users.filter(u => u.age > 18).length}
string_concat: ${"Hello, " + name}

# Component calls
simple_call:
  from!: base
  text: Click me

complex_call:
  yx-from: base_component
  user:
    name: $user_name
    profile:
      avatar: $avatar_url
  settings:
    theme: dark
    notifications: true

# Arrays and objects
array: [1, 2, 3, "four", true]
object:
  key1: value1
  key2: value2
  nested:
    deep: value

# Mixed content
mixed:
  literal: Hello
  ref: $name
  expr: ${1 + 2}
  call:
    from!: base
    prop: value
  array: [1, 2, 3]
"#;

pub const EDGE_CASE_YAML: &str = r#"
# Unicode and special characters
unicode: 🚀 Rocket
chinese: 你好世界
arabic: مرحبا بالعالم
emoji_complex: 🎨🖼️🎭🎪

# Escaped characters
escaped: "Hello \"World\""
newlines: "Line1\nLine2"
tabs: "Col1\tCol2"

# Empty and null values
empty_string: ""
null_value: null
empty_array: []
empty_object: {}

# Large numbers
large_int: 9223372036854775807
large_float: 1.7976931348623157e+308
negative_large: -9223372036854775808

# Scientific notation
scientific_pos: 1.23e+10
scientific_neg: 4.56e-5

# Special floats
inf_pos: .inf
inf_neg: -.Inf
nan: .NaN

# Timestamps
iso_date: 2023-12-25
iso_datetime: 2023-12-25T15:30:00Z

# Binary data
binary: !!binary |
  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5

# Custom tags
custom_tag: !custom_tag value
another_custom: !another:some:type complex_value

# Complex nesting
very_nested:
  level1:
    level2:
      level3:
        level4:
          level5:
            level6:
              deep_value: finally here

# Long component name
very_long_component_name_that_tests_length_limits_and_handling: value

# Special property names
"dollar.prefixed": $value
"curly.braced": ${expression}
"with spaces": some value
"with.dots": another value
"#;

pub const INVALID_YAML: &str = r#"
invalid: [unclosed array
another: unclosed "string
bad_indent:
    wrong indent level
"#;

// Component test utilities
pub fn assert_literal_component(component: &YMXComponent, expected_value: &str) {
    match &component.value {
        ComponentValue::Literal(s) => assert_eq!(s, expected_value),
        _ => panic!("Expected Literal component, got {:?}", component.value),
    }
}

pub fn assert_property_reference_component(component: &YMXComponent, expected_property: &str) {
    match &component.value {
        ComponentValue::PropertyReference(s) => assert_eq!(s, expected_property),
        _ => panic!("Expected PropertyReference component, got {:?}", component.value),
    }
}

pub fn assert_processing_context_component(component: &YMXComponent, expected_expr: &str) {
    match &component.value {
        ComponentValue::ProcessingContext(s) => assert_eq!(s, expected_expr),
        _ => panic!("Expected ProcessingContext component, got {:?}", component.value),
    }
}

pub fn assert_component_call(component: &YMXComponent, expected_target: &str, expected_props: &HashMap<String, ComponentValue>) {
    match &component.value {
        ComponentValue::ComponentCall(call) => {
            assert_eq!(call.target, expected_target);
            assert_eq!(call.properties.len(), expected_props.len());
            for (key, value) in &call.properties {
                assert_eq!(expected_props.get(key), Some(value));
            }
        },
        _ => panic!("Expected ComponentCall component, got {:?}", component.value),
    }
}

// Performance test utilities
pub fn measure_time<F, T>(f: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

// Memory test utilities (simplified)
pub fn estimate_memory_usage<T>(_item: &T) -> usize {
    std::mem::size_of::<T>()
}

// Test data generators
pub fn generate_test_yaml(num_components: usize) -> String {
    let mut yaml = String::new();
    for i in 0..num_components {
        yaml.push_str(&format!("component_{}: value_{}\n", i, i));
    }
    yaml
}

pub fn generate_test_components(num_components: usize) -> Vec<YMXComponent> {
    (0..num_components)
        .map(|i| create_test_component(
            &format!("component_{}", i),
            ComponentValue::Literal(format!("value_{}", i)),
        ))
        .collect()
}

// WASM test utilities
#[cfg(target_arch = "wasm32")]
pub fn setup_wasm_test() {
    console_error_panic_hook::set_once();
}

// Async test utilities
pub async fn async_test_component(component: &YMXComponent) -> Result<String, String> {
    // Simulate async work
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    component::execute_component(component, &create_test_context())
}

// Error test utilities
pub fn assert_parse_error(content: &str, expected_error_pattern: &str) {
    let result = parse_yaml_content(content);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains(expected_error_pattern), 
           "Expected error pattern '{}' not found in: '{}'", expected_error_pattern, error_msg);
}

pub fn assert_execution_error(component: &YMXComponent, context: &HashMap<String, String>, expected_error_pattern: &str) {
    let result = component::execute_component(component, context);
    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains(expected_error_pattern), 
           "Expected error pattern '{}' not found in: '{}'", expected_error_pattern, error_msg);
}

// Regression test utilities
pub fn run_regression_tests() {
    // Add known regression tests here
    let test_cases = vec![
        ("simple", SIMPLE_YAML, 3),
        ("property_refs", PROPERTY_REF_YAML, 3),
        ("processing_context", PROCESSING_CONTEXT_YAML, 4),
        ("component_calls", COMPONENT_CALL_YAML, 3),
        ("complex", COMPLEX_YAML, 12),
    ];
    
    for (name, yaml, expected_count) in test_cases {
        let result = parse_yaml_content(yaml);
        assert!(result.is_ok(), "Failed to parse {}: {:?}", name, result);
        let components = result.unwrap();
        assert_eq!(components.len(), expected_count, 
                  "Expected {} components in {}, got {}", expected_count, name, components.len());
    }
}

// Benchmark utilities
pub fn benchmark_parsing(iterations: usize, content: &str) -> (std::time::Duration, usize) {
    let start = std::time::Instant::now();
    let mut total_components = 0;
    
    for _ in 0..iterations {
        let result = parse_yaml_content(content);
        assert!(result.is_ok());
        total_components += result.unwrap().len();
    }
    
    (start.elapsed(), total_components)
}

pub fn benchmark_execution(iterations: usize, component: &YMXComponent, context: &HashMap<String, String>) -> std::time::Duration {
    let start = std::time::Instant::now();
    
    for _ in 0..iterations {
        let result = component::execute_component(component, context);
        assert!(result.is_ok());
    }
    
    start.elapsed()
}