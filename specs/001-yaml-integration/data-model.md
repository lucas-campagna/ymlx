# Data Model: YAML Integration System

**Feature**: YAML Integration System  
**Date**: 2026-01-12  
**Purpose**: Core data structures and entity definitions for YMX component processing

## Core Entities

### YMXComponent

Represents a single component definition in YMX language.

```rust
pub struct YMXComponent {
    pub id: ComponentId,
    pub name: String,
    pub value: ComponentValue,
    pub metadata: ComponentMetadata,
    pub location: SourceLocation,
}

pub struct ComponentId(String);

pub enum ComponentValue {
    Literal(String),
    PropertyReference(String),           // $property_name
    ProcessingContext(String),            // ${javascript_code}
    ComponentCall(ComponentCall),         // from!, yx-from, From
    Template(String),                    // Generic component with ~ prefix
}

pub struct ComponentCall {
    pub target: String,
    pub properties: HashMap<String, ComponentValue>,
}

pub struct ComponentMetadata {
    pub is_template: bool,               // $ prefix
    pub is_generic: bool,                // ~ prefix
    pub interpreter: Option<Interpreter>,  // JavaScript or Python
    pub dependencies: Vec<String>,         // Called components
}

pub enum Interpreter {
    JavaScript,
    Python,
}
```

### ExecutionEnvironment

Contains context for component execution with properties and interpreter state.

```rust
pub struct ExecutionEnvironment {
    pub properties: HashMap<String, Value>,
    pub interpreter: Interpreter,
    pub memory_limit: usize,
    pub time_limit: Duration,
    pub security_policy: SecurityPolicy,
}

pub struct SecurityPolicy {
    pub max_memory: usize,
    pub max_execution_time: Duration,
    pub allowed_apis: HashSet<String>,
    pub allow_file_access: bool,
    pub allow_network_access: bool,
}
```

### ParseResult

Represents the result of parsing YMX files with detailed error information.

```rust
pub struct ParseResult {
    pub components: Vec<YMXComponent>,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseWarning>,
    pub metadata: ParseMetadata,
}

pub struct ParseError {
    pub message: String,
    pub location: SourceLocation,
    pub error_type: ErrorType,
}

pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub span: usize,
}

pub enum ErrorType {
    SyntaxError,
    InvalidPropertyReference,
    CircularDependency,
    InterpreterError,
    SecurityViolation,
}
```

### ComponentLibrary

Manages collections of YMX components with dependency resolution.

```rust
pub struct ComponentLibrary {
    pub components: HashMap<String, YMXComponent>,
    pub dependencies: DependencyGraph,
    pub metadata: LibraryMetadata,
}

pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

pub struct DependencyNode {
    pub component_id: String,
    pub dependents: Vec<String>,
    pub dependencies: Vec<String>,
}

pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: DependencyType,
}

pub enum DependencyType {
    DirectCall,          // explicit from!, yx-from
    PropertyReference,    // $property usage
    ProcessingContext,   // interpreter execution
}
```

## Value Types

### Value

Represents runtime values that can flow through component execution.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(Box<dyn Callable>),  // For interpreter functions
}

impl Value {
    pub fn as_string(&self) -> Option<&str>;
    pub fn as_number(&self) -> Option<f64>;
    pub fn as_bool(&self) -> Option<bool>;
    pub fn as_array(&self) -> Option<&[Value]>;
    pub fn as_object(&self) -> Option<&HashMap<String, Value>>;
    
    pub fn is_truthy(&self) -> bool;
    pub fn merge(&self, other: &Value) -> Result<Value, MergeError>;
}
```

### PropertyContext

Manages property substitution and merging operations.

```rust
pub struct PropertyContext {
    pub properties: HashMap<String, Value>,
    pub default_value: Option<Value>,
    pub merge_strategy: MergeStrategy,
}

pub enum MergeStrategy {
    ObjectMerge,      // Use .. key for object merging
    ArrayConcat,       // Prepend .. to arrays
    Replace,          // Direct replacement
}

pub struct MergeOperation {
    pub target: String,
    pub source: Value,
    pub operation_type: MergeOperationType,
}

pub enum MergeOperationType {
    ObjectSpread,      // ..: property
    ArrayPrepend,      // ..property in arrays
    DirectAssignment,
}
```

## Configuration Models

### CLI Configuration

Configuration structure for command-line interface.

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct CliConfig {
    pub default_output_format: OutputFormat,
    pub timeout_seconds: u64,
    pub memory_limit_mb: usize,
    pub component_paths: Vec<PathBuf>,
    pub security_policy: SecurityPolicyConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityPolicyConfig {
    pub allow_file_access: bool,
    pub allow_network_access: bool,
    pub max_execution_time_ms: u64,
    pub max_memory_mb: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
    Plain,
}
```

### WASM Configuration

Configuration for web assembly deployment.

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct WasmConfig {
    pub enable_features: WasmFeatures,
    pub performance_mode: PerformanceMode,
    pub security_level: SecurityLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WasmFeatures {
    pub simd: bool,
    pub threads: bool,
    pub bulk_memory: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PerformanceMode {
    Size,           // Optimize for bundle size
    Speed,          // Optimize for execution speed
    Balanced,       // Balance between size and speed
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SecurityLevel {
    Restricted,     // Most secure, limited APIs
    Standard,       // Balanced security
    Permissive,     // Less secure, more features
}
```

## Validation Rules

### Component Validation

Rules for validating YMX component definitions.

```rust
pub struct ComponentValidator {
    pub allow_circular_dependencies: bool,
    pub max_nesting_depth: usize,
    pub max_component_size: usize,
}

impl ComponentValidator {
    pub fn validate(&self, component: &YMXComponent) -> ValidationResult;
    pub fn validate_library(&self, library: &ComponentLibrary) -> ValidationResult;
}

pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub location: SourceLocation,
    pub severity: ValidationSeverity,
}

pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}
```

## State Transitions

### Component Processing States

Lifecycle states for component processing.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingState {
    Pending,
    Parsing,
    Validating,
    ResolvingDependencies,
    Executing,
    Completed,
    Failed(String),
}

pub struct ProcessingContext {
    pub state: ProcessingState,
    pub component: YMXComponent,
    pub environment: ExecutionEnvironment,
    pub result: Option<Value>,
    pub errors: Vec<ExecutionError>,
}

impl ProcessingContext {
    pub fn transition_to(&mut self, new_state: ProcessingState) -> Result<(), TransitionError>;
    pub fn is_terminal(&self) -> bool;
}
```

## Error Types

### Execution Errors

Comprehensive error handling for component execution.

```rust
#[derive(Error, Debug)]
pub enum YmxError {
    #[error("Parse error: {message} at {location}")]
    ParseError { message: String, location: SourceLocation },
    
    #[error("Component not found: {component_id}")]
    ComponentNotFound { component_id: String },
    
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: Vec<String> },
    
    #[error("Execution timeout: component exceeded {limit}")]
    ExecutionTimeout { limit: Duration },
    
    #[error("Memory limit exceeded: used {used}, limit {limit}")]
    MemoryLimitExceeded { used: usize, limit: usize },
    
    #[error("Security violation: {violation}")]
    SecurityViolation { violation: String },
    
    #[error("Interpreter error: {error}")]
    InterpreterError { error: String },
    
    #[error("Property reference invalid: {property}")]
    InvalidPropertyReference { property: String },
}
```

## Relationships

### Entity Relationships

- **YMXComponent** 1..* -> **ComponentValue** (composition)
- **YMXComponent** 0..* -> **ComponentCall** (dependency)
- **ComponentLibrary** 1..* -> **YMXComponent** (aggregation)
- **ExecutionEnvironment** 1 -> **SecurityPolicy** (composition)
- **ParseResult** 0..* -> **ParseError** (composition)
- **DependencyGraph** 1..* -> **DependencyNode** (composition)

### Data Flow

1. **Input**: YAML files -> **YMXComponent** (parsing)
2. **Validation**: **YMXComponent** -> **ValidationResult** (validation)
3. **Execution**: **YMXComponent** + **ExecutionEnvironment** -> **Value** (processing)
4. **Output**: **Value** -> formatted output (CLI/WASM)

This data model provides a comprehensive foundation for implementing the YAML Integration System while maintaining parser-first design principles and performance-critical requirements.