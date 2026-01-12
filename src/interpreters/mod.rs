use crate::models::{ComponentValue, Value, Interpreter};
use crate::error::{YmxError, Result};
use std::collections::HashMap;

/// Trait for interpreter implementations
pub trait InterpreterEngine {
    fn execute(&self, code: &str, context: &HashMap<String, Value>) -> Result<Value>;
    fn set_timeout(&mut self, timeout: std::time::Duration);
    fn set_memory_limit(&mut self, limit: usize);
    fn is_available(&self) -> bool;
}

/// JavaScript interpreter using Boa engine
#[cfg(feature = "javascript")]
pub struct JavaScriptInterpreter {
    engine: boa_engine::Context,
    timeout: std::time::Duration,
    memory_limit: usize,
}

#[cfg(feature = "javascript")]
impl JavaScriptInterpreter {
    pub fn new() -> Self {
        let engine = boa_engine::Context::default();
        Self {
            engine,
            timeout: std::time::Duration::from_secs(5),
            memory_limit: 10 * 1024 * 1024, // 10MB
        }
    }
}

#[cfg(feature = "javascript")]
impl InterpreterEngine for JavaScriptInterpreter {
    fn execute(&self, code: &str, context: &HashMap<String, Value>) -> Result<Value> {
        // Set up context variables
        for (key, value) in context {
            let js_value = convert_value_to_js(value)?;
            self.engine.register_global_property(key.as_str(), js_value);
        }
        
        // Execute the code
        let result = std::thread::spawn({
            let code = code.to_string();
            let mut engine = self.engine.clone();
            move || {
                engine.eval(boa_engine::Source::from_bytes(&code))
            }
        });
        
        let timeout = std::time::timeout(self.timeout, result);
        match timeout {
            Ok(Ok(value)) => convert_js_to_value(&value),
            Ok(Err(e)) => Err(YmxError::InterpreterError { 
                error: format!("JavaScript execution failed: {}", e) 
            }),
            Err(_) => Err(YmxError::InterpreterError { 
                error: format!("Operation timed out after {:?}", self.timeout) 
            }),
        }
    }
    
    fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }
    
    fn set_memory_limit(&mut self, limit: usize) {
        self.memory_limit = limit;
        // TODO: Implement memory limiting in Boa
    }
    
    fn is_available(&self) -> bool {
        true // JavaScript interpreter is always available when feature is enabled
    }
}

/// Python interpreter using PyO3
#[cfg(feature = "python")]
pub struct PythonInterpreter {
    timeout: std::time::Duration,
    memory_limit: usize,
}

#[cfg(feature = "python")]
impl PythonInterpreter {
    pub fn new() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(5),
            memory_limit: 10 * 1024 * 1024, // 10MB
        }
    }
}

#[cfg(feature = "python")]
impl InterpreterEngine for PythonInterpreter {
    fn execute(&self, code: &str, context: &HashMap<String, Value>) -> Result<Value> {
        pyo3::prepare_freethreaded_python();
        
        Python::with_gil(|py| {
            let globals = pyo3::types::PyDict::new(py);
            
            // Set up context variables
            for (key, value) in context {
                let py_value = convert_value_to_py(py, value)?;
                globals.set_item(key, py_value)?;
            }
            
            // Execute the code
            let result = std::thread::spawn({
                let code = code.to_string();
                move || {
                    py.run(code.as_ref(), Some(&globals), None)
                }
            });
            
            let timeout = std::time::timeout(self.timeout, result);
            match timeout {
                Ok(Ok(value)) => {
                    let converted = convert_py_to_value(py, &value)?;
                    Ok(converted)
                },
                Ok(Err(e)) => Err(YmxError::InterpreterError { 
                    error: format!("Python execution failed: {}", e) 
                }),
                Err(_) => Err(YmxError::InterpreterError { 
                    error: format!("Operation timed out after {:?}", self.timeout) 
                }),
            }
        })
    }
    
    fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }
    
    fn set_memory_limit(&mut self, limit: usize) {
        self.memory_limit = limit;
        // TODO: Implement memory limiting in PyO3
    }
    
    fn is_available(&self) -> bool {
        true // Python interpreter is always available when feature is enabled
    }
}

/// No-op interpreter for when features are disabled
#[cfg(not(any(feature = "javascript", feature = "python")))]
pub struct NoOpInterpreter;

#[cfg(not(any(feature = "javascript", feature = "python")))]
impl InterpreterEngine for NoOpInterpreter {
    fn execute(&self, _code: &str, _context: &HashMap<String, Value>) -> Result<Value> {
        Err(YmxError::InterpreterError { 
            error: "No interpreter enabled. Enable 'javascript' or 'python' feature.".to_string() 
        })
    }
    
    fn set_timeout(&mut self, _timeout: std::time::Duration) {}
    
    fn set_memory_limit(&mut self, _limit: usize) {}
    
    fn is_available(&self) -> bool {
        false
    }
}

/// Interpreter factory
pub fn create_interpreter(interpreter: &Interpreter) -> Result<Box<dyn InterpreterEngine>> {
    match interpreter {
        #[cfg(feature = "javascript")]
        Interpreter::JavaScript => Ok(Box::new(JavaScriptInterpreter::new())),
        
        #[cfg(feature = "python")]
        Interpreter::Python => Ok(Box::new(PythonInterpreter::new())),
        
        #[cfg(not(feature = "javascript"))]
        Interpreter::JavaScript => Err(YmxError::InterpreterError { 
            error: "JavaScript interpreter not enabled. Build with 'javascript' feature.".to_string() 
        }),
        
        #[cfg(not(feature = "python"))]
        Interpreter::Python => Err(YmxError::InterpreterError { 
            error: "Python interpreter not enabled. Build with 'python' feature.".to_string() 
        }),
    }
}

// Helper functions for value conversion
#[cfg(feature = "javascript")]
fn convert_value_to_js(value: &Value) -> Result<boa_engine::JsValue> {
    match value {
        Value::Null => Ok(boa_engine::JsValue::null()),
        Value::Bool(b) => Ok(boa_engine::JsValue::boolean(*b)),
        Value::Number(n) => Ok(boa_engine::JsValue::new_number(*n)),
        Value::String(s) => Ok(boa_engine::JsValue::new_string(s.clone().into())),
        _ => Err(YmxError::InterpreterError { 
            error: "Unsupported value type for JavaScript conversion".to_string() 
        }),
    }
}

#[cfg(feature = "javascript")]
fn convert_js_to_value(value: &boa_engine::JsValue) -> Result<Value> {
    if value.is_undefined() {
        Ok(Value::Null)
    } else if value.is_null() {
        Ok(Value::Null)
    } else if value.is_boolean() {
        Ok(Value::Bool(value.to_boolean()))
    } else if let Some(n) = value.to_number() {
        Ok(Value::Number(n))
    } else if value.is_string() {
        Ok(Value::String(value.to_string().to_string()))
    } else {
        // Object and Array conversion - simplified for now
        Ok(Value::String("complex_value".to_string()))
    }
}

#[cfg(feature = "python")]
fn convert_value_to_py(py: pyo3::Python, value: &Value) -> Result<pyo3::Py<pyo3::PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => Ok(n.to_object(py)),
        Value::String(s) => Ok(s.to_object(py)),
        _ => Err(YmxError::InterpreterError { 
            error: "Unsupported value type for Python conversion".to_string() 
        }),
    }
}

#[cfg(feature = "python")]
fn convert_py_to_value(py: pyo3::Python, value: &pyo3::PyAny) -> Result<Value> {
    if value.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = value.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(n) = value.extract::<f64>() {
        Ok(Value::Number(n))
    } else if let Ok(s) = value.extract::<String>() {
        Ok(Value::String(s))
    } else {
        Err(YmxError::InterpreterError { 
            error: "Unsupported Python value type".to_string() 
        })
    }
}