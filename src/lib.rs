use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ComponentValue {
    Literal(String),
    PropertyReference(String),           
    ProcessingContext(String),            
    ComponentCall(ComponentCall),         
    Template(String),                    
}

#[derive(Debug, Clone)]
pub struct ComponentCall {
    pub target: String,
    pub properties: HashMap<String, ComponentValue>,
}

#[derive(Debug, Clone)]
pub struct YMXComponent {
    pub id: String,
    pub name: String,
    pub value: ComponentValue,
}

/// Simplified parsing for basic implementation
pub fn parse_yaml_content(content: &str) -> Result<Vec<YMXComponent>, String> {
    let mut components = Vec::new();
    let yaml: serde_yaml::Value = serde_yaml::from_str(content)
        .map_err(|e| format!("YAML parsing error: {}", e))?;
    
    if let serde_yaml::Value::Mapping(mapping) = yaml {
        for (key, value) in mapping {
            if let Some(key_str) = key.as_str() {
                let component_name = key_str.to_string();
                let component_value = parse_yaml_value(value)?;
                
                components.push(YMXComponent {
                    id: component_name.clone(),
                    name: component_name.clone(),
                    value: component_value,
                });
            }
        }
    }
    
    Ok(components)
}

fn parse_yaml_value(value: &serde_yaml::Value) -> Result<ComponentValue, String> {
    match value {
        serde_yaml::Value::String(s) => Ok(ComponentValue::Literal(s.clone())),
        serde_yaml::Value::Mapping(mapping) => {
            // Check for component calls or properties
            let has_from = mapping.contains_key("from!") || 
                            mapping.contains_key("yx-from") || 
                            mapping.contains_key("From");
            let has_properties = mapping.keys().any(|k| {
                let k_str = k.as_str().unwrap_or("");
                !k_str.starts_with('$') && k_str != "from!" && k_str != "yx-from" && k_str != "From"
            });
            
            if has_from && has_properties {
                // Component call
                let target = extract_target(&mapping)?;
                let mut properties = HashMap::new();
                
                for (k, v) in mapping {
                    if let Some(k_str) = k.as_str() {
                        if k_str != "from!" && k_str != "yx-from" && k_str != "From" {
                            if let Ok(val) = parse_yaml_value(v) {
                                properties.insert(k_str.to_string(), val);
                            }
                        }
                    }
                }
                
                Ok(ComponentValue::ComponentCall(ComponentCall {
                    target,
                    properties,
                }))
            } else if has_properties {
                // Plain component with properties
                let mut properties = HashMap::new();
                for (k, v) in mapping {
                    if let Some(k_str) = k.as_str() {
                        if let Ok(val) = parse_yaml_value(v) {
                            properties.insert(k_str.to_string(), val);
                        }
                    }
                }
                
                Ok(ComponentValue::Literal(serde_yaml::to_string(&serde_yaml::Value::Object(&properties))?))
            } else {
                // Simple literal or processing context
                let component_str = serde_yaml::to_string(&serde_yaml::Value::Mapping(mapping))?;
                
                if component_str.contains("${") && component_str.contains('}') {
                    Ok(ComponentValue::ProcessingContext(component_str))
                } else if component_str.starts_with('$') {
                    Ok(ComponentValue::PropertyReference(component_str.trim_start_matches('$').unwrap().to_string()))
                } else {
                    Ok(ComponentValue::Literal(component_str))
                }
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            let array_value: Result<Vec<ComponentValue>, String> = seq
                .iter()
                .map(|v| parse_yaml_value(v))
                .collect();
            
            Ok(ComponentValue::Literal(serde_yaml::to_string(&serde_yaml::Value::Array(&array_value?))?))
        }
        serde_yaml::Value::Bool(b) => Ok(ComponentValue::Literal(b.to_string())),
        serde_yaml::Value::Number(n) => Ok(ComponentValue::Literal(n.to_string())),
        serde_yaml::Value::Null => Ok(ComponentValue::Literal("null".to_string())),
        _ => Err(format!("Unsupported YAML value type: {:?}", value)),
    }
}

fn extract_target(mapping: &serde_yaml::Mapping) -> Result<String, String> {
    for (key, _) in mapping {
        if let Some(key_str) = key.as_str() {
            if key_str == "from!" || key_str == "yx-from" || key_str == "From" {
                if let Some(target) = mapping.get("from!") {
                    if let Some(s) = target.as_str() {
                        return Ok(s.to_string().clone());
                    }
                }
            }
        }
    }
    
    Err("No valid target found in component call".to_string())
}

pub mod component {
    use super::*;
    
    pub fn execute_component(
        component: &YMXComponent,
        context: &HashMap<String, String>,
    ) -> Result<String, String> {
        let mut result_context = context.clone();
        
        // Substitute properties
        if let Ok(substituted_value) = substitute_properties(&component.value, &result_context) {
            if let ComponentValue::Literal(content) = substituted_value {
                return Ok(content);
            } else if let ComponentValue::ProcessingContext(code) = substituted_value {
                // Simple evaluation for demonstration
                if code.contains('+') || code.contains('-') || code.contains('*') || code.contains('/') {
                    Ok(format!("Evaluated: {}", code))
                } else {
                    Ok("Processing context evaluation not implemented".to_string())
                }
            } else if let ComponentValue::ComponentCall(call) = substituted_value {
                return Ok(format!("Called component: {} with params: {:?}", call.target, call.properties));
            } else {
                Ok("Unknown component type".to_string())
            }
        } else {
            Err("Component substitution failed".to_string())
        }
    }
}

fn substitute_properties(
    value: &ComponentValue,
    context: &HashMap<String, String>,
) -> Result<ComponentValue, String> {
    match value {
        ComponentValue::Literal(content) => Ok(ComponentValue::Literal(content.clone())),
        ComponentValue::PropertyReference(property) => {
            if let Some(replacement) = context.get(property) {
                Ok(ComponentValue::Literal(replacement.clone()))
            } else {
                Err(format!("Property '{}' not found in context", property))
            }
        },
        ComponentValue::ProcessingContext(code) => {
            let substituted_code = substitute_in_string(code, context);
            Ok(ComponentValue::ProcessingContext(substituted_code))
        },
        _ => Err("Property substitution failed".to_string()),
    }
}

fn substitute_in_string(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '{' {
                    // Processing context ${...}
                    let (context_expr, _) = extract_processing_context(&mut chars)?;
                    result.push_str(&format!("${{{}}}", context_expr));
                    chars = _;
                } else {
                    // Simple property reference $property
                    let (property_name, _) = extract_property_name(&mut chars)?;
                    if let Some(replacement) = context.get(&property_name) {
                        result.push_str(&replacement);
                    } else {
                        result.push('$');
                    }
                    chars = _;
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

fn extract_processing_context(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(String, std::iter::Peekable<std::str::Chars>)> {
    let mut expression = String::new();
    let mut consumed_chars = Vec::new();
    let brace_count = 1; // We've already consumed ${
    
    consumed_chars.push('{');
    
    while brace_count > 0 {
        if let Some(ch) = chars.next() {
            consumed_chars.push(ch);
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => expression.push(ch),
            }
        } else {
            break;
        }
    }
    
    if brace_count > 0 {
        return Err("Unterminated processing context".to_string());
    }
    
    Ok((expression, consumed_chars.into_iter().chain(chars).collect()))
}

fn extract_property_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(String, std::iter::Peekable<std::str::Chars>)> {
    let mut name = String::new();
    let mut consumed_chars = Vec::new();
    
    while let Some(ch) = chars.next() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
            consumed_chars.push(ch);
        } else {
            // Put back the non-property character
            return Ok((name, std::iter::once('$').chain(consumed_chars.into_iter()).collect()));
        }
    }
    
    Ok((name, consumed_chars.into_iter().chain(chars).collect()))
}

// Error type for simplified implementation
#[derive(Debug)]
pub enum ComponentError {
    ParseError(String),
    SubstitutionError(String),
    ExecutionError(String),
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ComponentError::SubstitutionError(msg) => write!(f, "Substitution error: {}", msg),
            ComponentError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for ComponentError {
    fn source(&self) -> Option<std::error::Error> {
        match self {
            ComponentError::ParseError(_) => None,
            ComponentError::SubstitutionError(_) => None,
            ComponentError::ExecutionError(_) => None,
        }
    }
}