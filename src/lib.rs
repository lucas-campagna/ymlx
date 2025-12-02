pub mod parser;
pub mod renderes;

use parser::runtime::Runtime;
use rust_yaml::{Yaml, Value};

pub enum Input<'a> {
    Json(&'a Value),
    Code(&'a str),
}

pub fn build(components: Input) -> Result<Runtime, Box<dyn std::error::Error>> {
    match components {
        Input::Json(components) => {
            let mut runtime = Runtime::new();
            let _ = runtime.add_many(components);
            Ok(runtime)
        },
        Input::Code(code) => {
            let yaml = Yaml::new();
            let components = yaml.load_str(code)?;
            let mut runtime = Runtime::new();
            let _ = runtime.add_many(&components);
            Ok(runtime)
        }
    }
}