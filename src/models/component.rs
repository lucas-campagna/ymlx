use crate::error::{Result, YmxError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Component ID wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub String);

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Component value types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum ComponentValue {
    Literal { content: String },
    PropertyReference { property: String }, // $property_name
    ProcessingContext { code: String },     // ${javascript_code}
    ComponentCall(ComponentCall),           // from!, yx-from, From
    Template { pattern: String },           // Generic component with ~ prefix
}

/// Component call structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCall {
    pub target: String,
    pub properties: HashMap<String, ComponentValue>,
}

/// Component metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    pub is_template: bool,                // $ prefix
    pub is_generic: bool,                 // ~ prefix
    pub interpreter: Option<Interpreter>, // JavaScript or Python
    pub dependencies: Vec<String>,        // Called components
}

/// Interpreter types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Interpreter {
    JavaScript,
    Python,
}

/// Core YMX component structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YMXComponent {
    pub id: ComponentId,
    pub name: String,
    pub value: ComponentValue,
    pub metadata: ComponentMetadata,
    pub location: SourceLocation,
}

/// Source location for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub span: usize,
}

impl SourceLocation {
    pub fn new(file: impl Into<PathBuf>, line: usize, column: usize, span: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            span,
        }
    }

    pub fn display(&self) -> String {
        format!("{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Execution environment for component processing
#[derive(Debug, Clone)]
pub struct ExecutionEnvironment {
    pub properties: HashMap<String, Value>,
    pub interpreter: Interpreter,
    pub memory_limit: usize,
    pub time_limit: Duration,
    pub security_policy: SecurityPolicy,
}

/// Security policy for sandboxed execution
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub max_memory: usize,
    pub max_execution_time: Duration,
    pub allowed_apis: std::collections::HashSet<String>,
    pub allow_file_access: bool,
    pub allow_network_access: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            max_memory: 10 * 1024 * 1024, // 10MB
            max_execution_time: Duration::from_secs(5),
            allowed_apis: std::collections::HashSet::new(),
            allow_file_access: false,
            allow_network_access: false,
        }
    }
}

/// Runtime values that flow through component execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
        }
    }

    pub fn merge(&self, other: &Value) -> Result<Value> {
        match (self, other) {
            (Value::Object(mut obj1), Value::Object(obj2)) => {
                for (key, value) in obj2 {
                    obj1.insert(key.clone(), value.clone());
                }
                Ok(Value::Object(obj1))
            }
            (Value::Array(mut arr1), Value::Array(arr2)) => {
                arr1.extend(arr2.clone());
                Ok(Value::Array(arr1))
            }
            _ => Ok(other.clone()),
        }
    }
}

/// Property context for substitutions
#[derive(Debug, Clone)]
pub struct PropertyContext {
    pub properties: HashMap<String, Value>,
    pub default_value: Option<Value>,
    pub merge_strategy: MergeStrategy,
}

/// Merge strategy for properties
#[derive(Debug, Clone)]
pub enum MergeStrategy {
    ObjectMerge, // Use .. key for object merging
    ArrayConcat, // Prepend .. to arrays
    Replace,     // Direct replacement
}

/// Component library for managing collections
#[derive(Debug, Clone)]
pub struct ComponentLibrary {
    pub components: HashMap<String, YMXComponent>,
    pub dependencies: DependencyGraph,
    pub metadata: LibraryMetadata,
}

/// Library metadata
#[derive(Debug, Clone)]
pub struct LibraryMetadata {
    pub total_components: usize,
    pub created_at: Option<std::time::SystemTime>,
    pub updated_at: Option<std::time::SystemTime>,
}

/// Dependency graph for component relationships
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

/// Dependency node
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub component_id: String,
    pub dependents: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Dependency edge
#[derive(Debug, Clone)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: DependencyType,
}

/// Type of dependency
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyType {
    DirectCall,        // explicit from!, yx-from
    PropertyReference, // $property usage
    ProcessingContext, // interpreter execution
}
