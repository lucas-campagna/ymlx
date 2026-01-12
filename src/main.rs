use clap::Parser;
use ymx::{parse_yaml_content};
use ymx::component::execute_component;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name = "ymx")]
#[command(about = "YMX component integration system", long_about = None)]
#[command(version = "0.1.0")]
struct Args {
    /// The component to call
    caller: String,

    /// YAML file containing components
    file: String,

    /// Properties to pass to the component (format: key=value)
    #[arg(short = 'p', long, value_parser = parse_key_val)]
    property: Vec<(String, String)>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

/// Parse a single key-value pair
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s.find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() {
    let args = Args::parse();
    
    if args.verbose {
        eprintln!("Calling component: {}", args.caller);
        eprintln!("Using file: {}", args.file);
        if !args.property.is_empty() {
            eprintln!("Properties:");
            for (key, value) in &args.property {
                eprintln!("  {}={}", key, value);
            }
        }
    }
    
    // Convert properties to HashMap
    let mut context = HashMap::new();
    for (key, value) in args.property {
        context.insert(key, value);
    }
    context.insert("caller".to_string(), args.caller.clone());
    
    // Parse and execute component
    match std::fs::read_to_string(&args.file) {
        Ok(content) => {
            match parse_yaml_content(&content) {
                Ok(components) => {
                    if let Some(component) = components.iter().find(|c| c.name == args.caller) {
                        match execute_component(component, &context) {
                            Ok(result) => {
                                println!("{}", result);
                                std::process::exit(0);
                            },
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("Component '{}' not found in file", args.caller);
                        eprintln!("Available components:");
                        for component in &components {
                            eprintln!("  - {}", component.name);
                        }
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("File read error: {}", e);
            std::process::exit(1);
        }
    }
}