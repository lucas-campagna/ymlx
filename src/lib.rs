use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentValue {
    Literal(String),
    PropertyReference(String),           
    ProcessingContext(String),            
    ComponentCall(ComponentCall),         
    Template(String),                    
}

#[derive(Debug, Clone, PartialEq)]
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
                let component_value = parse_yaml_value(&value)?;
                
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
                
                Ok(ComponentValue::Literal(serde_yaml::to_string(&serde_yaml::Value::Mapping(
                    serde_yaml::Mapping::from_iter(
                        properties.iter().map(|(k, v)| {
                            let yaml_value = match v {
                                ComponentValue::Literal(s) => serde_yaml::Value::String(s.clone()),
                                ComponentValue::PropertyReference(s) => serde_yaml::Value::String(format!("${}", s)),
                                ComponentValue::ProcessingContext(s) => serde_yaml::Value::String(s.clone()),
                                ComponentValue::ComponentCall(_) => serde_yaml::Value::String("[ComponentCall]".to_string()),
                                ComponentValue::Template(s) => serde_yaml::Value::String(s.clone()),
                            };
                            (serde_yaml::Value::String(k.clone()), yaml_value)
                        })
                    )
                )).map_err(|e| e.to_string())?))
            } else {
                // Simple literal or processing context
                let component_str = serde_yaml::to_string(&serde_yaml::Value::Mapping(mapping.clone())).map_err(|e| e.to_string())?;
                
                if component_str.contains("${") && component_str.contains('}') {
                    Ok(ComponentValue::ProcessingContext(component_str))
                } else if component_str.starts_with('$') {
                    Ok(ComponentValue::PropertyReference(component_str.trim_start_matches('$').to_string()))
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
            
            let yaml_array = serde_yaml::Sequence::from_iter(
                array_value?.into_iter().map(|cv| match cv {
                    ComponentValue::Literal(s) => serde_yaml::Value::String(s),
                    ComponentValue::PropertyReference(s) => serde_yaml::Value::String(format!("${}", s)),
                    ComponentValue::ProcessingContext(s) => serde_yaml::Value::String(s),
                    ComponentValue::ComponentCall(_) => serde_yaml::Value::String("[ComponentCall]".to_string()),
                    ComponentValue::Template(s) => serde_yaml::Value::String(s),
                })
            );
            Ok(ComponentValue::Literal(serde_yaml::to_string(&serde_yaml::Value::Sequence(yaml_array)).map_err(|e| e.to_string())?))
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
        let result_context = context.clone();
        
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
            let substituted_code = substitute_in_string(code, context)?;
            Ok(ComponentValue::ProcessingContext(substituted_code))
        },
        _ => Err("Property substitution failed".to_string()),
    }
}

pub fn substitute_in_string(template: &str, context: &HashMap<String, String>) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '{' {
                    // Processing context ${...}
                    chars.next(); // consume '{'
                    let context_expr = extract_processing_context(&mut chars)?;
                    result.push_str(&format!("${{{}}}", context_expr));
                } else {
                    // Simple property reference $property
                    let (property_name, terminating_char) = extract_property_name_with_terminator(&mut chars)?;
                    if let Some(replacement) = context.get(&property_name) {
                        result.push_str(&replacement);
                    } else {
                        result.push('$');
                        result.push_str(&property_name);
                    }
                    // Add back the terminating character
                    if let Some(ch) = terminating_char {
                        result.push(ch);
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

fn extract_processing_context(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
    let mut expression = String::new();
    let mut brace_count = 1; // We've already consumed ${
    
    while brace_count > 0 {
        if let Some(ch) = chars.next() {
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
    
    Ok(expression)
}

fn extract_property_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
    let mut name = String::new();
    
    while let Some(ch) = chars.next() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
        } else {
            // Put back the non-property character
            return Ok(name);
        }
    }
    
    Ok(name)
}

fn extract_property_name_with_terminator(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(String, Option<char>), String> {
    let mut name = String::new();
    
    while let Some(ch) = chars.next() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
        } else {
            // Return property name and the terminating character
            return Ok((name, Some(ch)));
        }
    }
    
    Ok((name, None))
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
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComponentError::ParseError(_) => None,
            ComponentError::SubstitutionError(_) => None,
            ComponentError::ExecutionError(_) => None,
        }
    }
}

// WASM bindings for web interface
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct YMXProcessor {
    components: Vec<YMXComponent>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl YMXProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(yaml_content: &str) -> Result<YMXProcessor, JsValue> {
        let components = parse_yaml_content(yaml_content)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(YMXProcessor { components })
    }
    
    #[wasm_bindgen]
    pub fn get_component_names(&self) -> JsValue {
        let names: Vec<String> = self.components.iter()
            .map(|c| c.name.clone())
            .collect();
        JsValue::from_serde(&names).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn execute_component(&self, component_name: &str, properties: JsValue) -> Result<String, JsValue> {
        let context: std::collections::HashMap<String, String> = properties
            .into_serde()
            .map_err(|e| JsValue::from_str(&format!("Invalid properties: {}", e)))?;
        
        if let Some(component) = self.components.iter()
            .find(|c| c.name == component_name) {
            component::execute_component(component, &context)
                .map_err(|e| JsValue::from_str(&e))
        } else {
            Err(JsValue::from_str(&format!("Component '{}' not found", component_name)))
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn process_component_yaml(yaml_content: &str, component_name: &str, properties_array: JsValue) -> Result<String, JsValue> {
    let processor = YMXProcessor::new(yaml_content)?;
    processor.execute_component(component_name, properties_array)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn validate_yaml_syntax(yaml_content: &str) -> Result<(), JsValue> {
    parse_yaml_content(yaml_content)
        .map(|_| ())
        .map_err(|e| JsValue::from_str(&e))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_yaml_components(yaml_content: &str) -> Result<String, JsValue> {
    let components = parse_yaml_content(yaml_content)
        .map_err(|e| JsValue::from_str(&e))?;
    
    let component_names: Vec<String> = components.iter()
        .map(|c| c.name.clone())
        .collect();
    
    serde_json::to_string(&component_names)
        .map_err(|e| JsValue::from_str(&e))
}