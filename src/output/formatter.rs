use crate::models::Value;
use crate::config::OutputFormat;
use serde_json;
use tabled::{Tabled, Table};
use colored::*;

/// Output formatter for different formats
pub struct OutputFormatter {
    format: OutputFormat,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }
    
    /// Print a value using the configured format
    pub fn print_value(&self, value: &Value) -> Result<(), crate::error::YmxError> {
        match self.format {
            OutputFormat::Json => self.print_json(value),
            OutputFormat::Yaml => self.print_yaml(value),
            OutputFormat::Table => self.print_table(value),
            OutputFormat::Plain => self.print_plain(value),
        }
    }
    
    /// Print as JSON
    fn print_json(&self, value: &Value) -> Result<(), crate::error::YmxError> {
        let json = serde_json::to_string_pretty(value)
            .map_err(|e| crate::error::YmxError::ParseError {
                message: format!("JSON serialization error: {}", e),
                source: Box::new(crate::error::YmxError::InvalidPropertyReference {
                    property: "json_serialization".to_string(),
                }),
                location: crate::models::SourceLocation::new("", 0, 0, 0),
            })?;
        
        println!("{}", json);
        Ok(())
    }
    
    /// Print as YAML
    fn print_yaml(&self, value: &Value) -> Result<(), crate::error::YmxError> {
        let yaml = serde_yaml::to_string(value)
            .map_err(|e| crate::error::YmxError::ParseError {
                message: format!("YAML serialization error: {}", e),
                source: Box::new(crate::error::YmxError::InvalidPropertyReference {
                    property: "yaml_serialization".to_string(),
                }),
                location: crate::models::SourceLocation::new("", 0, 0, 0),
            })?;
        
        println!("{}", yaml);
        Ok(())
    }
    
    /// Print as table (for objects and arrays)
    fn print_table(&self, value: &Value) -> Result<(), crate::error::YmxError> {
        match value {
            Value::Object(obj) => {
                if obj.is_empty() {
                    println!("No data to display");
                    return Ok(());
                }
                
                // Convert to table format
                let rows: Vec<TableRow> = obj
                    .iter()
                    .map(|(key, val)| TableRow {
                        key: key.clone(),
                        value: self.value_to_string(val),
                        type_name: self.value_type_name(val),
                    })
                    .collect();
                
                let table = Table::new(&rows).to_string();
                println!("{}", table);
            },
            Value::Array(arr) => {
                if arr.is_empty() {
                    println!("No data to display");
                    return Ok(());
                }
                
                for (index, value) in arr.iter().enumerate() {
                    println!("{}: {}", index + 1, self.value_to_string(value));
                }
            },
            _ => {
                println!("{}", self.value_to_string(value));
            }
        }
        
        Ok(())
    }
    
    /// Print as plain text
    fn print_plain(&self, value: &Value) -> Result<(), crate::error::YmxError> {
        println!("{}", self.value_to_string(value));
        Ok(())
    }
    
    /// Convert Value to string representation
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let strings: Vec<String> = arr.iter().map(|v| self.value_to_string(v)).collect();
                format!("[{}]", strings.join(", "))
            },
            Value::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_string(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            },
        }
    }
    
    /// Get type name for a value
    fn value_type_name(&self, value: &Value) -> String {
        match value {
            Value::String(_) => "string".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Null => "null".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        }
    }
}

/// Table row for object display
#[derive(Tabled)]
struct TableRow {
    #[tabled(rename = "Key")]
    key: String,
    
    #[tabled(rename = "Value")]
    value: String,
    
    #[tabled(rename = "Type")]
    type_name: String,
}