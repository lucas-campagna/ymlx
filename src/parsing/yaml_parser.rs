use crate::models::{
    ComponentId, ComponentValue, YMXComponent, ComponentMetadata,
    ComponentCall, Interpreter, SourceLocation, Value
};
use crate::error::{YmxError, Result};
use crate::utils::logging::{PerformanceTimer, get_logger};
use crate::config::performance;
use serde_yaml;
use std::collections::HashMap;
use std::path::Path;

/// YAML parser for YMX components
pub struct YamlParser {
    logger: crate::utils::logging::Logger,
}

impl YamlParser {
    pub fn new() -> Self {
        Self {
            logger: get_logger(),
        }
    }
    
    /// Parse YMX content from string
    pub fn parse_content(&self, content: &str, file_path: &Path) -> Result<ParseResult> {
        let timer = PerformanceTimer::new("parsing", Duration::from_millis(performance::PARSE_TIME_LIMIT_MS));
        
        let parse_result = self.parse_yaml_content(content, file_path)?;
        
        let elapsed = timer.finish(&self.logger)?;
        self.logger.debug(&format!("Parsed {} components in {}ms", parse_result.components.len(), elapsed.as_millis()));
        
        Ok(parse_result)
    }
    
    /// Parse YMX content from file
    pub fn parse_file(&self, file_path: &Path) -> Result<ParseResult> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| YmxError::IoError(e))?;
        
        self.parse_content(&content, file_path)
    }
    
    fn parse_yaml_content(&self, content: &str, file_path: &Path) -> Result<ParseResult> {
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(content)
            .map_err(|e| YmxError::YamlSyntaxError {
                message: e.to_string(),
                line: 0, // TODO: Extract line info from serde_yaml
                column: 0,
            })?;
        
        let components = self.extract_components(&yaml_value, file_path)?;
        let errors = Vec::new(); // TODO: Collect parsing errors
        let warnings = Vec::new(); // TODO: Collect warnings
        
        Ok(ParseResult {
            components,
            errors,
            warnings,
            metadata: ParseMetadata {
                total_components: components.len(),
                parse_time_ms: 0, // TODO: Use timer result
            },
        })
    }
    
    fn extract_components(&self, yaml: &serde_yaml::Value, file_path: &Path) -> Result<Vec<YMXComponent>> {
        let mut components = Vec::new();
        
        match yaml {
            serde_yaml::Value::Mapping(mapping) => {
                for (key, value) in mapping {
                    let key_str = key.as_str().unwrap_or("");
                    let component = self.create_component(key_str, value, file_path)?;
                    components.push(component);
                }
            },
            _ => {
                return Err(YmxError::ParseError {
                    message: "YAML must be a mapping of component definitions".to_string(),
                    source: Box::new(YmxError::YamlSyntaxError {
                        message: "Invalid YAML structure".to_string(),
                        line: 1,
                        column: 1,
                    }),
                    location: SourceLocation::new(file_path, 1, 1, 0),
                });
            }
        }
        
        Ok(components)
    }
    
    fn create_component(&self, name: &str, value: &serde_yaml::Value, file_path: &Path) -> Result<YMXComponent> {
        let component_value = self.parse_component_value(value, name, file_path, 1, 1)?;
        let metadata = self.extract_metadata(name, &component_value, file_path)?;
        
        Ok(YMXComponent {
            id: ComponentId(name.to_string()),
            name: name.to_string(),
            value: component_value,
            metadata,
            location: SourceLocation::new(file_path, 1, 1, 0), // TODO: Extract actual location
        })
    }
    
    fn parse_component_value(&self, value: &serde_yaml::Value, component_name: &str, file_path: &Path, line: usize, column: usize) -> Result<ComponentValue> {
        match value {
            serde_yaml::Value::String(s) => {
                // Check for different component value types
                if s.starts_with("${") && s.ends_with('}') {
                    // Processing context ${...}
                    Ok(ComponentValue::ProcessingContext {
                        code: s[2..s.len()-1].to_string(),
                    })
                } else if s.starts_with('$') && !s.starts_with("${") {
                    // Property reference $property
                    Ok(ComponentValue::PropertyReference {
                        property: s[1..].to_string(),
                    })
                } else {
                    // Literal value
                    Ok(ComponentValue::Literal {
                        content: s.clone(),
                    })
                }
            },
            serde_yaml::Value::Mapping(mapping) => {
                // Component call or template
                self.parse_mapping_component(mapping, component_name, file_path, line, column)
            },
            serde_yaml::Value::Sequence(sequence) => {
                // Array literal
                let array_value: Result<Vec<Value>> = sequence
                    .iter()
                    .map(|v| self.yaml_to_value(v, file_path, line, column))
                    .collect();
                Ok(ComponentValue::Literal {
                    content: serde_json::to_string(&array_value?).unwrap_or_default(),
                })
            },
            serde_yaml::Value::Number(n) => {
                Ok(ComponentValue::Literal {
                    content: n.to_string(),
                })
            },
            serde_yaml::Value::Bool(b) => {
                Ok(ComponentValue::Literal {
                    content: b.to_string(),
                })
            },
            serde_yaml::Value::Null => {
                Ok(ComponentValue::Literal {
                    content: "null".to_string(),
                })
            },
        }
    }
    
    fn parse_mapping_component(&self, mapping: &serde_yaml::Mapping, component_name: &str, file_path: &Path, line: usize, column: usize) -> Result<ComponentValue> {
        // Check for component call properties
        let mut component_call = None;
        let mut properties = HashMap::new();
        
        for (key, value) in mapping {
            let key_str = key.as_str().unwrap_or("");
            
            if self.is_component_call_key(key_str) {
                let target = self.extract_target_from_yaml_value(value)?;
                component_call = Some(ComponentCall {
                    target,
                    properties: HashMap::new(), // Will be filled below
                });
            } else if key_str != ".." {
                // Regular property
                let prop_value = self.yaml_to_value(value, file_path, line, column)?;
                properties.insert(key_str.to_string(), prop_value);
            }
        }
        
        // If we found a component call, create ComponentCall
        if let Some(call) = component_call {
            Ok(ComponentValue::ComponentCall(call))
        } else {
            // Regular object literal
            let object_value = self.mapping_to_hashmap(mapping, file_path, line, column)?;
            Ok(ComponentValue::Literal {
                content: serde_json::to_string(&object_value).unwrap_or_default(),
            })
        }
    }
    
    fn is_component_call_key(&self, key: &str) -> bool {
        matches!(key, "from!" | "yx-from" | "From")
    }
    
    fn extract_target_from_yaml_value(&self, value: &serde_yaml::Value) -> Result<String> {
        match value {
            serde_yaml::Value::String(s) => Ok(s.clone()),
            serde_yaml::Value::Mapping(_) => {
                // Complex target extraction - for now assume first key is target
                Err(YmxError::ParseError {
                    message: "Complex component calls not yet supported".to_string(),
                    source: Box::new(YmxError::InvalidPropertyReference {
                        property: "component_call_target".to_string(),
                    }),
                    location: SourceLocation::new("", 0, 0, 0),
                })
            },
            _ => Err(YmxError::ParseError {
                message: "Invalid component call target".to_string(),
                source: Box::new(YmxError::InvalidPropertyReference {
                    property: "component_call_target".to_string(),
                }),
                location: SourceLocation::new("", 0, 0, 0),
            }),
        }
    }
    
    fn yaml_to_value(&self, yaml: &serde_yaml::Value, file_path: &Path, line: usize, column: usize) -> Result<Value> {
        match yaml {
            serde_yaml::Value::String(s) => Ok(Value::String(s.clone())),
            serde_yaml::Value::Number(n) => Ok(Value::Number(n.as_f64().unwrap_or(0.0))),
            serde_yaml::Value::Bool(b) => Ok(Value::Bool(*b)),
            serde_yaml::Value::Null => Ok(Value::Null),
            serde_yaml::Value::Sequence(seq) => {
                let array: Result<Vec<Value>> = seq
                    .iter()
                    .map(|v| self.yaml_to_value(v, file_path, line, column))
                    .collect();
                Ok(Value::Array(array?))
            },
            serde_yaml::Value::Mapping(map) => {
                let object = self.mapping_to_hashmap(map, file_path, line, column)?;
                Ok(Value::Object(object))
            },
        }
    }
    
    fn mapping_to_hashmap(&self, mapping: &serde_yaml::Mapping, file_path: &Path, line: usize, column: usize) -> Result<HashMap<String, Value>> {
        let mut hashmap = HashMap::new();
        
        for (key, value) in mapping {
            let key_str = key.as_str().unwrap_or("");
            let value_converted = self.yaml_to_value(value, file_path, line, column)?;
            hashmap.insert(key_str.to_string(), value_converted);
        }
        
        Ok(hashmap)
    }
    
    fn extract_metadata(&self, name: &str, value: &ComponentValue, file_path: &Path) -> Result<ComponentMetadata> {
        let is_template = name.starts_with('$');
        let is_generic = name.starts_with('~');
        let interpreter = self.detect_interpreter(value)?;
        let dependencies = self.extract_dependencies(value);
        
        Ok(ComponentMetadata {
            is_template,
            is_generic,
            interpreter,
            dependencies,
        })
    }
    
    fn detect_interpreter(&self, value: &ComponentValue) -> Result<Option<Interpreter>> {
        match value {
            ComponentValue::ProcessingContext { code } => {
                // Simple heuristic: if code looks like JavaScript, assume JS
                if code.contains("function") || code.contains("const") || code.contains("let") {
                    Ok(Some(Interpreter::JavaScript))
                } else if code.contains("def ") || code.contains("import ") {
                    Ok(Some(Interpreter::Python))
                } else {
                    Ok(None)
                }
            },
            _ => Ok(None),
        }
    }
    
    fn extract_dependencies(&self, value: &ComponentValue) -> Vec<String> {
        match value {
            ComponentValue::ComponentCall(call) => {
                let mut deps = vec![call.target.clone()];
                deps.extend(call.properties.values()
                    .filter_map(|v| self.extract_component_name_from_value(v))
                );
                deps
            },
            ComponentValue::ProcessingContext { code } => {
                // Extract component calls from processing context
                // This is simplified - real implementation would parse the code
                code.split_whitespace()
                    .filter(|word| word.chars().all(|c| c.is_alphanumeric() || c == '_'))
                    .filter(|word| !["if", "for", "while", "function", "const", "let"].contains(word))
                    .map(|s| s.to_string())
                    .collect()
            },
            ComponentValue::PropertyReference { property } => {
                // Check if property reference might be a component
                if !property.is_empty() {
                    vec![property.clone()]
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new(),
        }
    }
    
    fn extract_component_name_from_value(&self, value: &ComponentValue) -> Option<String> {
        match value {
            ComponentValue::ComponentCall(call) => Some(call.target.clone()),
            ComponentValue::PropertyReference { property } => Some(property.clone()),
            ComponentValue::ProcessingContext { code } => {
                // Simplified - extract potential component names
                code.split_whitespace()
                    .find(|word| word.chars().all(|c| c.is_alphanumeric() || c == '_'))
                    .map(|s| s.to_string())
            },
            _ => None,
        }
    }
}

/// Result of parsing operation
pub struct ParseResult {
    pub components: Vec<YMXComponent>,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseWarning>,
    pub metadata: ParseMetadata,
}

/// Parse metadata
pub struct ParseMetadata {
    pub total_components: usize,
    pub parse_time_ms: u64,
}

/// Parse error (different from YmxError)
pub struct ParseError {
    pub message: String,
    pub location: SourceLocation,
    pub error_type: ErrorType,
}

/// Parse warning
pub struct ParseWarning {
    pub message: String,
    pub location: SourceLocation,
    pub warning_type: WarningType,
}

/// Error types for parsing
#[derive(Debug, Clone)]
pub enum ErrorType {
    SyntaxError,
    InvalidPropertyReference,
    CircularDependency,
    InterpreterError,
    SecurityViolation,
}

/// Warning types for parsing
#[derive(Debug, Clone)]
pub enum WarningType {
    UnusedProperty,
    DeprecatedSyntax,
    PerformanceIssue,
}

impl Default for YamlParser {
    fn default() -> Self {
        Self::new()
    }
}