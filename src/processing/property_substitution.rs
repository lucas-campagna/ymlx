use crate::models::{ComponentValue, Value};
use crate::error::{YmxError, Result};
use std::collections::HashMap;

/// Property substitution engine for YMX components
pub struct PropertySubstitution {
    max_nesting_depth: usize,
}

impl PropertySubstitution {
    pub fn new() -> Self {
        Self {
            max_nesting_depth: 50, // Prevent infinite recursion
        }
    }
    
    /// Substitute properties in a component value
    pub fn substitute(&self, value: &ComponentValue, context: &HashMap<String, Value>) -> Result<ComponentValue> {
        match value {
            ComponentValue::Literal { content } => {
                let substituted = self.substitute_in_string(content, context, 0)?;
                Ok(ComponentValue::Literal { content: substituted })
            },
            ComponentValue::PropertyReference { property } => {
                self.substitute_property_reference(property, context)
            },
            ComponentValue::ProcessingContext { code } => {
                let substituted_code = self.substitute_in_string(code, context, 0)?;
                Ok(ComponentValue::ProcessingContext { code: substituted_code })
            },
            ComponentValue::ComponentCall(call) => {
                let mut substituted_call = call.clone();
                for (key, value) in &mut substituted_call.properties {
                    *value = self.substitute(value, context)?;
                }
                Ok(ComponentValue::ComponentCall(substituted_call))
            },
            ComponentValue::Template { pattern } => {
                Ok(ComponentValue::Template { pattern: pattern.clone() })
            },
        }
    }
    
    /// Substitute properties in a string
    fn substitute_in_string(&self, input: &str, context: &HashMap<String, Value>, depth: usize) -> Result<String> {
        if depth > self.max_nesting_depth {
            return Err(YmxError::ParseError {
                message: "Property substitution depth exceeded limit".to_string(),
                source: Box::new(YmxError::InvalidPropertyReference {
                    property: "nesting_depth".to_string(),
                }),
                location: crate::models::SourceLocation::new("", 0, 0, 0),
            });
        }
        
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '$' => {
                    // Check for property reference
                    if let Some(next_char) = chars.peek() {
                        if *next_char == '{' {
                            // Processing context ${...}
                            let (context_expr, consumed) = self.extract_processing_context(&mut chars)?;
                            result.push_str(&format!("${{{}}", context_expr));
                            chars = consumed;
                        } else {
                            // Simple property reference $property
                            let (property_name, consumed) = self.extract_property_name(&mut chars)?;
                            if let Some(value) = context.get(&property_name) {
                                result.push_str(&self.value_to_string(value)?);
                            } else {
                                return Err(YmxError::InvalidPropertyReference {
                                    property: property_name,
                                });
                            }
                            chars = consumed;
                        }
                    } else {
                        result.push('$');
                    }
                },
                _ => {
                    result.push(ch);
                }
            }
        }
        
        Ok(result)
    }
    
    /// Extract property name after $ symbol
    fn extract_property_name(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(String, std::iter::Peekable<std::str::Chars>)> {
        let mut name = String::new();
        let mut consumed_chars = Vec::new();
        
        while let Some(ch) = chars.next() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
                consumed_chars.push(ch);
            } else {
                // Put back the non-property character
                return Ok((name, std::iter::once(ch).chain(consumed_chars.into_iter()).chain(chars).collect()));
            }
        }
        
        Ok((name, std::iter::once('$').chain(consumed_chars.into_iter()).collect()))
    }
    
    /// Extract processing context expression between ${ and }
    fn extract_processing_context(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(String, std::iter::Peekable<std::str::Chars>)> {
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
                    _ => {}
                }
                if brace_count > 0 {
                    expression.push(ch);
                }
            } else {
                return Err(YmxError::ParseError {
                    message: "Unterminated processing context".to_string(),
                    source: Box::new(YmxError::InvalidPropertyReference {
                        property: "processing_context".to_string(),
                    }),
                    location: crate::models::SourceLocation::new("", 0, 0, 0),
                });
            }
        }
        
        Ok((expression, consumed_chars.into_iter().chain(chars).collect()))
    }
    
    /// Handle property reference substitution
    fn substitute_property_reference(&self, property: &str, context: &HashMap<String, Value>) -> Result<ComponentValue> {
        match context.get(property) {
            Some(value) => self.value_to_component_value(value),
            None => Err(YmxError::InvalidPropertyReference {
                property: property.to_string(),
            }),
        }
    }
    
    /// Convert Value to string representation
    fn value_to_string(&self, value: &Value) -> Result<String> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
            Value::Array(arr) => {
                let strings: Result<Vec<String>> = arr
                    .iter()
                    .map(|v| self.value_to_string(v))
                    .collect();
                Ok(format!("[{}]", strings?.join(", ")))
            },
            Value::Object(obj) => {
                let pairs: Result<Vec<String>> = obj
                    .iter()
                    .map(|(k, v)| {
                        let value_str = self.value_to_string(v)?;
                        Ok(format!("\"{}\": {}", k, value_str))
                    })
                    .collect();
                Ok(format!("{{{}}}", pairs?.join(", ")))
            },
        }
    }
    
    /// Convert Value to ComponentValue
    fn value_to_component_value(&self, value: &Value) -> ComponentValue {
        match value {
            Value::String(s) => ComponentValue::Literal { content: s.clone() },
            Value::Number(n) => ComponentValue::Literal { content: n.to_string() },
            Value::Bool(b) => ComponentValue::Literal { content: b.to_string() },
            Value::Null => ComponentValue::Literal { content: "null".to_string() },
            Value::Array(_) | Value::Object(_) => {
                // Complex values become JSON literals
                ComponentValue::Literal { 
                    content: serde_json::to_string(value).unwrap_or_default() 
                }
            },
        }
    }
    
    /// Handle object merging with .. key
    pub fn merge_objects(&self, target: &mut HashMap<String, Value>, source: &HashMap<String, Value>) -> Result<()> {
        for (key, value) in source {
            if key == ".." {
                // Object spread operation
                if let Value::Object(source_obj) = value {
                    for (spread_key, spread_value) in source_obj {
                        target.insert(spread_key.clone(), spread_value.clone());
                    }
                } else {
                    return Err(YmxError::ParseError {
                        message: "Object spread (..) must be used with an object".to_string(),
                        source: Box::new(YmxError::InvalidPropertyReference {
                            property: "object_spread".to_string(),
                        }),
                        location: crate::models::SourceLocation::new("", 0, 0, 0),
                    });
                }
            } else {
                target.insert(key.clone(), value.clone());
            }
        }
        Ok(())
    }
}

impl Default for PropertySubstitution {
    fn default() -> Self {
        Self::new()
    }
}