use crate::models::{ComponentValue, Interpreter, Value};
use crate::error::{YmxError, Result};
use std::collections::HashMap;
use std::time::Duration;

/// Trait for interpreter implementations
pub trait InterpreterEngine {
    fn execute(&self, code: &str, context: &HashMap<String, Value>) -> Result<Value>;
    fn set_timeout(&mut self, timeout: Duration);
    fn set_memory_limit(&mut self, limit: usize);
    fn is_available(&self) -> bool;
}

/// JavaScript interpreter using Boa engine
#[cfg(feature = "javascript")]
pub struct JavaScriptInterpreter {
    engine: boa_engine::Context,
    timeout: Duration,
    memory_limit: usize,
}

#[cfg(feature = "javascript")]
impl JavaScriptInterpreter {
    pub fn new() -> Self {
        let engine = boa_engine::Context::default();
        Self {
            engine,
            timeout: Duration::from_secs(5),
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
        
        // Execute the code with timeout
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
            Err(_) => Err(YmxError::ExecutionTimeout { 
                limit: format!("{:?}", self.timeout) 
            }),
        }
    }
    
    fn set_timeout(&mut self, timeout: Duration) {
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
    timeout: Duration,
    memory_limit: usize,
}

#[cfg(feature = "python")]
impl PythonInterpreter {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(5),
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
            
            // Execute the code with timeout
            let result = std::thread::spawn({
                let code = code.to_string();
                move || {
                    let result = py.run(code.as_ref(), Some(&globals), None);
                    result
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
                Err(_) => Err(YmxError::ExecutionTimeout { 
                    limit: format!("{:?}", self.timeout) 
                }),
            }
        })
    }
    
    fn set_timeout(&mut self, timeout: Duration) {
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
    
    fn set_timeout(&mut self, _timeout: Duration) {}
    
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
        Value::Bool(b) => Ok(boa_engine::JsValue::Boolean(*b)),
        Value::Number(n) => Ok(boa_engine::JsValue::Rational(*n)),
        Value::String(s) => Ok(boa_engine::JsValue::String(s.clone().into())),
        Value::Array(arr) => {
            let js_array = boa_engine::JsArray::new_array_default(0);
            for item in arr {
                let js_item = convert_value_to_js(item)?;
                js_array.push(js_item, false).unwrap();
            }
            Ok(js_array.into())
        },
        Value::Object(obj) => {
            let js_object = boa_engine::JsObject::from_proto_and_data(boa_engine::JsPrototype::object(), None);
            for (key, value) in obj {
                let js_value = convert_value_to_js(value)?;
                js_object.set(key.as_str(), js_value, false).unwrap();
            }
            Ok(js_object.into())
        },
    }
}

#[cfg(feature = "javascript")]
fn convert_js_to_value(value: &boa_engine::JsValue) -> Result<Value> {
    if value.is_null() {
        Ok(Value::Null)
    } else if value.is_boolean() {
        Ok(Value::Bool(value.to_boolean()))
    } else if value.is_number() {
        Ok(Value::Number(value.to_number()))
    } else if value.is_string() {
        Ok(Value::String(value.to_string().to_string()))
    } else if value.is_array() {
        let arr = value.to_object().unwrap();
        let length = arr.get("length", &boa_engine::Context::default()).unwrap().to_number() as usize;
        let mut result = Vec::new();
        for i in 0..length {
            let item = arr.get(i as u32, &boa_engine::Context::default()).unwrap();
            result.push(convert_js_to_value(&item)?);
        }
        Ok(Value::Array(result))
    } else {
        // Object conversion
        Ok(Value::Object(HashMap::new())) // Simplified for now
    }
}

#[cfg(feature = "python")]
fn convert_value_to_py(py: Python, value: &Value) -> Result<pyo3::Py<pyo3::PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => Ok(n.to_object(py)),
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = pyo3::types::PyList::empty(py);
            for item in arr {
                py_list.append(convert_value_to_py(py, item)?)?;
            }
            Ok(py_list.into())
        },
        Value::Object(obj) => {
            let py_dict = pyo3::types::PyDict::new(py);
            for (key, value) in obj {
                py_dict.set_item(key, convert_value_to_py(py, value)?)?;
            }
            Ok(py_dict.into())
        },
    }
}

#[cfg(feature = "python")]
fn convert_py_to_value(py: Python, value: &pyo3::PyAny) -> Result<Value> {
    if value.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = value.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(n) = value.extract::<f64>() {
        Ok(Value::Number(n))
    } else if let Ok(s) = value.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(arr) = value.downcast::<pyo3::types::PyList>() {
        let mut result = Vec::new();
        for item in arr.iter() {
            result.push(convert_py_to_value(py, &item?)?);
        }
        Ok(Value::Array(result))
    } else if let Ok(dict) = value.downcast::<pyo3::types::PyDict>() {
        let mut result = HashMap::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            let converted_value = convert_py_to_value(py, value?)?;
            result.insert(key_str, converted_value);
        }
        Ok(Value::Object(result))
    } else {
        Err(YmxError::InterpreterError { 
            error: "Unsupported Python value type".to_string() 
        })
    }
}