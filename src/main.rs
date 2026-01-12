use crate::lib::component::{execute_component, parse_yaml_content};
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: ymx <component_file> [--property key=value ...]");
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let mut context = HashMap::new();
    
    // Parse additional properties
    for arg in &args[2..] {
        if let Some((key, value)) = arg.split_once('=') {
            context.insert(key.to_string(), value.to_string());
        }
    }
    
    // Parse and execute component
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            match parse_yaml_content(content) {
                Ok(components) => {
                    if let Some(component) = components.first() {
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
                        eprintln!("No components found in file");
                        std::process::exit(1);
                    }
                }
                },
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