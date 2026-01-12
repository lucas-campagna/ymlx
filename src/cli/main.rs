use clap::{Parser, Subcommand};
use crate::models::Value;
use crate::error::{YmxError, Result};
use crate::config::{load_config, OutputFormat};
use crate::parsing::YamlParser;
use crate::execution::ExecutionEngine;
use crate::output::OutputFormatter;
use std::collections::HashMap;
use std::path::PathBuf;

/// YMX Component Integration System
#[derive(Parser)]
#[command(name = "ymx")]
#[command(about = "Execute YMX component files", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, help = "Output format")]
    pub output_format: Option<OutputFormat>,
    
    #[arg(short, long, help = "Configuration file")]
    pub config: Option<PathBuf>,
    
    #[arg(short = 'v', long, help = "Verbose output")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a YMX component file
    Run {
        #[arg(help = "Component file to execute")]
        file: PathBuf,
        
        #[arg(short, long, help = "Input parameters as JSON")]
        params: Vec<String>,
        
        #[arg(short, long, help = "Component to execute (default: first component)")]
        component: Option<String>,
        
        #[arg(long, help = "Timeout in seconds")]
        timeout: Option<u64>,
    },
    
    /// Parse and validate YMX files
    Parse {
        #[arg(help = "YMX files to parse")]
        files: Vec<PathBuf>,
        
        #[arg(short, long, help = "Strict validation mode")]
        strict: bool,
        
        #[arg(short, long, help = "Validate dependencies")]
        validate_deps: bool,
    },
    
    /// List available components in library
    List {
        #[arg(short, long, help = "Component paths to scan")]
        path: Vec<PathBuf>,
        
        #[arg(long, help = "Filter by type")]
        filter: Option<String>,
        
        #[arg(short, long, help = "Sort by field")]
        sort: Option<String>,
    },
    
    /// Validate component library
    Validate {
        #[arg(help = "Component directories to validate")]
        directories: Vec<PathBuf>,
        
        #[arg(long, help = "Maximum nesting depth")]
        max_depth: Option<usize>,
        
        #[arg(long, help = "Maximum component size in KB")]
        max_size: Option<usize>,
    },
}

/// Main CLI application
pub struct YmxCli {
    config: crate::config::Config,
    formatter: OutputFormatter,
    parser: YamlParser,
    executor: ExecutionEngine,
}

impl YmxCli {
    pub fn new(cli: Cli) -> Result<Self> {
        // Load configuration
        let mut config = load_config().map_err(|e| YmxError::ConfigurationError {
            message: format!("Failed to load config: {}", e),
        })?;
        
        // Override with CLI arguments
        if let Some(output_format) = cli.output_format {
            config.cli.default_output_format = output_format;
        }
        
        let formatter = OutputFormatter::new(config.cli.default_output_format.clone());
        let parser = YamlParser::new();
        let executor = ExecutionEngine::new();
        
        Ok(Self {
            config,
            formatter,
            parser,
            executor,
        })
    }
    
    /// Execute the CLI command
    pub fn run(&self, cli: Cli) -> Result<()> {
        match cli.command {
            Commands::Run { file, params, component, timeout } => {
                self.execute_run_command(file, params, component, timeout)
            },
            Commands::Parse { files, strict, validate_deps } => {
                self.execute_parse_command(files, strict, validate_deps)
            },
            Commands::List { path, filter, sort } => {
                self.execute_list_command(path, filter, sort)
            },
            Commands::Validate { directories, max_depth, max_size } => {
                self.execute_validate_command(directories, max_depth, max_size)
            },
        }
    }
    
    /// Execute run command
    fn execute_run_command(
        &self,
        file: std::path::PathBuf,
        params: Vec<String>,
        component: Option<String>,
        timeout: Option<u64>,
    ) -> Result<()> {
        // Parse the component file
        let parse_result = self.parser.parse_file(&file)?;
        
        if !parse_result.errors.is_empty() {
            eprintln!("Parse errors:");
            for error in parse_result.errors {
                eprintln!("  {}", error.message);
            }
            return Err(YmxError::ParseError {
                message: "File contains parse errors".to_string(),
                source: Box::new(YmxError::YamlSyntaxError {
                    message: "Parse errors found".to_string(),
                    line: 0,
                    column: 0,
                }),
                location: crate::models::SourceLocation::new(file, 0, 0, 0),
            });
        }
        
        // Parse input parameters
        let mut input_params = HashMap::new();
        for param in params {
            self.parse_param(&param, &mut input_params)?;
        }
        
        // Select component to execute
        let component_name = component.unwrap_or_else(|| {
            parse_result.components.first().map(|c| c.name.clone()).unwrap_or_default()
        });
        
        let component = parse_result.components
            .iter()
            .find(|c| c.name == component_name)
            .ok_or_else(|| YmxError::ComponentNotFound {
                component_id: component_name,
            })?;
        
        // Execute the component
        let result = self.executor.execute_component(component, &input_params)?;
        
        // Output the result
        self.formatter.print_value(&result)?;
        
        Ok(())
    }
    
    /// Execute parse command
    fn execute_parse_command(
        &self,
        files: Vec<std::path::PathBuf>,
        strict: bool,
        validate_deps: bool,
    ) -> Result<()> {
        let mut all_components = Vec::new();
        let mut all_errors = Vec::new();
        
        for file in files {
            let parse_result = self.parser.parse_file(&file)?;
            all_components.extend(parse_result.components);
            all_errors.extend(parse_result.errors);
            
            // Print file-specific results
            println!("File: {}", file.display());
            println!("Components: {}", parse_result.components.len());
            
            if !parse_result.errors.is_empty() {
                println!("Errors:");
                for error in parse_result.errors {
                    println!("  {}: {}", error.location.display(), error.message);
                }
            }
            
            if !parse_result.warnings.is_empty() {
                println!("Warnings:");
                for warning in parse_result.warnings {
                    println!("  {}: {}", warning.location.display(), warning.message);
                }
            }
            
            println!();
        }
        
        if strict && !all_errors.is_empty() {
            return Err(YmxError::ParseError {
                message: "Parse errors found in strict mode".to_string(),
                source: Box::new(YmxError::YamlSyntaxError {
                    message: "Strict validation failed".to_string(),
                    line: 0,
                    column: 0,
                }),
                location: crate::models::SourceLocation::new("", 0, 0, 0),
            });
        }
        
        if validate_deps {
            // TODO: Implement dependency validation
            println!("Dependency validation not yet implemented");
        }
        
        Ok(())
    }
    
    /// Execute list command
    fn execute_list_command(
        &self,
        paths: Vec<std::path::PathBuf>,
        filter: Option<String>,
        sort: Option<String>,
    ) -> Result<()> {
        // TODO: Implement component listing
        println!("List command not yet implemented");
        println!("Paths: {:?}", paths);
        if let Some(f) = filter {
            println!("Filter: {}", f);
        }
        if let Some(s) = sort {
            println!("Sort: {}", s);
        }
        
        Ok(())
    }
    
    /// Execute validate command
    fn execute_validate_command(
        &self,
        directories: Vec<std::path::PathBuf>,
        max_depth: Option<usize>,
        max_size: Option<usize>,
    ) -> Result<()> {
        // TODO: Implement validation
        println!("Validate command not yet implemented");
        println!("Directories: {:?}", directories);
        if let Some(depth) = max_depth {
            println!("Max depth: {}", depth);
        }
        if let Some(size) = max_size {
            println!("Max size: {}KB", size);
        }
        
        Ok(())
    }
    
    /// Parse parameter string into key-value pairs
    fn parse_param(&self, param: &str, params: &mut HashMap<String, Value>) -> Result<()> {
        if let Some(eq_pos) = param.find('=') {
            let key = &param[..eq_pos];
            let value_str = &param[eq_pos + 1..];
            
            let value = if value_str.starts_with('{') || value_str.starts_with('[') {
                // Try to parse as JSON
                serde_json::from_str(value_str)
                    .map_err(|e| YmxError::ParseError {
                        message: format!("Invalid JSON in parameter '{}': {}", param, e),
                        source: Box::new(YmxError::YamlSyntaxError {
                            message: "JSON parse error".to_string(),
                            line: 0,
                            column: 0,
                        }),
                        location: crate::models::SourceLocation::new("", 0, 0, 0),
                    })?
            } else {
                Value::String(value_str.to_string())
            };
            
            params.insert(key.to_string(), value);
        } else {
            // Simple boolean flag
            params.insert(param.to_string(), Value::Bool(true));
        }
        
        Ok(())
    }
}

/// Main entry point
pub fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose {
        crate::utils::logging::LogLevel::Debug
    } else {
        crate::utils::logging::LogLevel::Info
    };
    crate::utils::logging::init_logger(true, log_level);
    
    // Create and run CLI
    let ymx_cli = YmxCli::new(cli)?;
    ymx_cli.run(cli)
}