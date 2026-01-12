use figment::{
    providers::{Env, Json, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// CLI configuration structure
#[derive(Debug, Deserialize, Serialize)]
pub struct CliConfig {
    #[serde(default = "default_output_format")]
    pub default_output_format: OutputFormat,

    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    #[serde(default = "default_memory_limit")]
    pub memory_limit_mb: usize,

    #[serde(default)]
    pub component_paths: Vec<PathBuf>,

    #[serde(default)]
    pub security_policy: SecurityPolicyConfig,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            default_output_format: OutputFormat::default(),
            timeout_seconds: default_timeout(),
            memory_limit_mb: default_memory_limit(),
            component_paths: Vec::new(),
            security_policy: SecurityPolicyConfig::default(),
        }
    }
}

/// Security policy configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityPolicyConfig {
    #[serde(default)]
    pub allow_file_access: bool,

    #[serde(default)]
    pub allow_network_access: bool,

    #[serde(default = "default_max_execution_time")]
    pub max_execution_time_ms: u64,

    #[serde(default = "default_max_memory")]
    pub max_memory_mb: usize,
}

impl Default for SecurityPolicyConfig {
    fn default() -> Self {
        Self {
            allow_file_access: false,
            allow_network_access: false,
            max_execution_time_ms: default_max_execution_time(),
            max_memory_mb: default_max_memory(),
        }
    }
}

/// Output format options
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
    Plain,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

/// WASM configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct WasmConfig {
    #[serde(default)]
    pub enable_features: WasmFeatures,

    #[serde(default = "default_performance_mode")]
    pub performance_mode: PerformanceMode,

    #[serde(default = "default_security_level")]
    pub security_level: SecurityLevel,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            enable_features: WasmFeatures::default(),
            performance_mode: PerformanceMode::default(),
            security_level: SecurityLevel::default(),
        }
    }
}

/// WASM feature configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct WasmFeatures {
    #[serde(default)]
    pub simd: bool,

    #[serde(default)]
    pub threads: bool,

    #[serde(default = "default_bulk_memory")]
    pub bulk_memory: bool,
}

impl Default for WasmFeatures {
    fn default() -> Self {
        Self {
            simd: true,
            threads: false,
            bulk_memory: default_bulk_memory(),
        }
    }
}

/// Performance mode for WASM
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceMode {
    Size,
    Speed,
    Balanced,
}

impl Default for PerformanceMode {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Security level for WASM
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SecurityLevel {
    Restricted,
    Standard,
    Permissive,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Application configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub cli: CliConfig,
    pub wasm: WasmConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cli: CliConfig::default(),
            wasm: WasmConfig::default(),
        }
    }
}

/// Load configuration from multiple sources
pub fn load_config() -> figment::Result<Config> {
    Figment::new()
        // Start with environment variables
        .merge(Env::prefixed("YMX_"))
        // Override with config file
        .merge(Toml::file("ymx.toml"))
        .merge(Json::file("ymx.json"))
        // Use defaults for missing values
        .merge(Serialized::defaults(Config::default()))
        .extract()
}

/// Default value functions
fn default_output_format() -> OutputFormat {
    OutputFormat::Json
}

fn default_timeout() -> u64 {
    30
}

fn default_memory_limit() -> usize {
    100
}

fn default_max_execution_time() -> u64 {
    5000
}

fn default_max_memory() -> usize {
    10
}

fn default_performance_mode() -> PerformanceMode {
    PerformanceMode::Balanced
}

fn default_security_level() -> SecurityLevel {
    SecurityLevel::Standard
}

fn default_bulk_memory() -> bool {
    true
}

/// Performance constants derived from requirements
pub mod performance {
    pub const PARSE_TIME_LIMIT_MS: u64 = 100; // <100ms for <1KB components
    pub const ERROR_REPORTING_TIME_MS: u64 = 10; // <10ms for error reporting
    pub const MEMORY_LIMIT_MB: usize = 100; // <100MB memory limit
    pub const MAX_COMPONENT_SIZE_MB: usize = 10; // 10MB max file size
    pub const CLI_EXECUTION_TIMEOUT_S: u64 = 2; // <2s CLI execution
    pub const WASM_COMPILATION_TIMEOUT_S: u64 = 5; // <5s WASM compilation
}
