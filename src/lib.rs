pub mod config;
pub mod error;
pub mod models;
pub mod interpreters;
pub mod utils;

#[cfg(feature = "yaml")]
pub mod parsing;

#[cfg(feature = "yaml")]
pub mod processing;

#[cfg(feature = "yaml")]
pub mod execution;

#[cfg(feature = "yaml")]
pub mod output;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "library")]
pub mod library;

// Re-export main types
pub use error::{YmxError, Result, SourceLocation, ErrorSeverity};
pub use models::{
    ComponentId, ComponentValue, YMXComponent, ComponentMetadata, 
    ComponentCall, Interpreter, Value, ExecutionEnvironment,
    SecurityPolicy, PropertyContext, ComponentLibrary,
    DependencyGraph, DependencyNode, DependencyEdge,
    DependencyType, MergeStrategy
};
pub use config::{Config, CliConfig, WasmConfig, SecurityPolicyConfig};
pub use interpreters::{InterpreterEngine, create_interpreter};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "library")]
pub mod library;

// Re-export main types
pub use error::{YmxError, Result, SourceLocation, ErrorSeverity};
pub use error::reporting::ErrorReporter;
pub use models::{
    ComponentId, ComponentValue, YMXComponent, ComponentMetadata, 
    ComponentCall, Interpreter, Value, ExecutionEnvironment,
    SecurityPolicy, PropertyContext, ComponentLibrary,
    DependencyGraph, DependencyNode, DependencyEdge,
    DependencyType, MergeStrategy
};
pub use config::{Config, CliConfig, WasmConfig, SecurityPolicyConfig};
pub use interpreters::{InterpreterEngine, create_interpreter};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");