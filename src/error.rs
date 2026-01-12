use thiserror::Error;
use std::fmt;

/// Main error type for YMX processing
#[derive(Error, Debug)]
pub enum YmxError {
    #[error("Parse error: {message} at {location}")]
    ParseError { 
        message: String, 
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        location: SourceLocation 
    },
    
    #[error("Component not found: {component_id}")]
    ComponentNotFound { component_id: String },
    
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },
    
    #[error("Execution timeout: component exceeded {limit}")]
    ExecutionTimeout { limit: String },
    
    #[error("Memory limit exceeded: used {used}, limit {limit}")]
    MemoryLimitExceeded { used: usize, limit: usize },
    
    #[error("Security violation: {violation}")]
    SecurityViolation { violation: String },
    
    #[error("Interpreter error: {error}")]
    InterpreterError { error: String },
    
    #[error("Property reference invalid: {property}")]
    InvalidPropertyReference { property: String },
    
    #[error("YAML syntax error: {message} at line {line}, column {column}")]
    YamlSyntaxError { 
        message: String, 
        line: usize, 
        column: usize 
    },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Source location information for errors
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: std::path::PathBuf,
    pub line: usize,
    pub column: usize,
    pub span: usize,
}

impl SourceLocation {
    pub fn new(file: impl Into<std::path::PathBuf>, line: usize, column: usize, span: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            span,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Result type for YMX operations
pub type Result<T> = std::result::Result<T, YmxError>;

/// Error severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
}

/// Validation error for component validation
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid component syntax: {message}")]
    InvalidSyntax { message: String },
    
    #[error("Component size exceeds limit: {size} bytes, limit: {limit}")]
    SizeExceeded { size: usize, limit: usize },
    
    #[error("Nesting depth exceeds limit: {depth}, limit: {limit}")]
    NestingTooDeep { depth: usize, limit: usize },
    
    #[error("Invalid interpreter: {interpreter}")]
    InvalidInterpreter { interpreter: String },
}

/// Performance error type
#[derive(Error, Debug)]
pub enum PerformanceError {
    #[error("Parsing took too long: {duration_ms}ms, limit: {limit_ms}ms")]
    ParseTimeout { duration_ms: u64, limit_ms: u64 },
    
    #[error("Error reporting took too long: {duration_ms}ms, limit: {limit_ms}ms")]
    ErrorReportingTimeout { duration_ms: u64, limit_ms: u64 },
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub parse_time_ms: u64,
    pub error_reporting_time_ms: u64,
    pub memory_usage_mb: usize,
}