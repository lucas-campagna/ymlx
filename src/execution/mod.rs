use crate::models::{ComponentValue, Value, ExecutionEnvironment};
use crate::interpreters::{InterpreterEngine, create_interpreter};
use crate::utils::logging::{PerformanceTimer, get_logger};
use std::collections::HashMap;
use std::time::Duration;

/// Component execution engine
pub struct ExecutionEngine {
    logger: crate::utils::logging::Logger,
    substitution_engine: crate::processing::PropertySubstitution,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self {
            logger: get_logger(),
            substitution_engine: crate::processing::PropertySubstitution::new(),
        }
    }
    
    /// Execute a single component with given context
    pub fn execute_component(
        &self,
        component: &crate::models::YMXComponent,
        context: &HashMap<String, Value>,
    ) -> Result<Value> {
        let timer = PerformanceTimer::new("execution", Duration::from_secs(5));
        
        self.logger.debug(&format!("Executing component: {}", component.name));
        
        // Step 1: Substitute properties in component value
        let substituted_value = self.substitution_engine.substitute(&component.value, context)?;
        
        // Step 2: Execute based on value type
        let result = match substituted_value {
            ComponentValue::Literal { content } => {
                self.execute_literal(&content, context)
            },
            ComponentValue::PropertyReference { property } => {
                self.execute_property_reference(&property, context)
            },
            ComponentValue::ProcessingContext { code } => {
                self.execute_processing_context(&code, &component.metadata, context)
            },
            ComponentValue::ComponentCall(call) => {
                self.execute_component_call(&call, &component.metadata, context)
            },
            ComponentValue::Template { pattern } => {
                self.execute_template(&pattern, context)
            },
        };
        
        timer.finish(&self.logger)?;
        self.logger.debug(&format!("Component {} executed successfully", component.name));
        
        result
    }
    
    /// Execute literal value
    fn execute_literal(&self, content: &str, context: &HashMap<String, Value>) -> Result<Value> {
        // For literals, just return the content as a string value
        // In a more sophisticated implementation, we might parse JSON literals
        if content.trim().starts_with('{') || content.trim().starts_with('[') {
            // Try to parse as JSON
            match serde_json::from_str::<Value>(content) {
                Ok(value) => Ok(value),
                Err(_) => Ok(Value::String(content.to_string())),
            }
        } else {
            Ok(Value::String(content.to_string()))
        }
    }
    
    /// Execute property reference
    fn execute_property_reference(&self, property: &str, context: &HashMap<String, Value>) -> Result<Value> {
        context
            .get(property)
            .cloned()
            .ok_or_else(|| YmxError::InvalidPropertyReference {
                property: property.to_string(),
            })
    }
    
    /// Execute processing context with interpreter
    fn execute_processing_context(
        &self,
        code: &str,
        metadata: &crate::models::ComponentMetadata,
        context: &HashMap<String, Value>,
    ) -> Result<Value> {
        let interpreter = metadata.interpreter.as_ref().unwrap_or(&crate::models::Interpreter::JavaScript);
        let mut interpreter_engine = create_interpreter(interpreter)?;
        
        // Set security limits
        interpreter_engine.set_timeout(Duration::from_secs(5));
        interpreter_engine.set_memory_limit(10 * 1024 * 1024); // 10MB
        
        // Execute the code
        interpreter_engine.execute(code, context)
    }
    
    /// Execute component call
    fn execute_component_call(
        &self,
        call: &crate::models::ComponentCall,
        _metadata: &crate::models::ComponentMetadata,
        context: &HashMap<String, Value>,
    ) -> Result<Value> {
        // This is a simplified implementation
        // In a full implementation, we would:
        // 1. Load the target component
        // 2. Merge call properties with provided context
        // 3. Execute the target component
        
        self.logger.debug(&format!("Calling component: {}", call.target));
        
        // For now, return a mock result indicating the call
        let mut merged_context = context.clone();
        for (key, value) in &call.properties {
            merged_context.insert(key.clone(), value.clone());
        }
        
        // Mock execution result
        Ok(Value::String(format!("Result of calling {}", call.target)))
    }
    
    /// Execute template pattern
    fn execute_template(&self, pattern: &str, _context: &HashMap<String, Value>) -> Result<Value> {
        // Template execution would involve:
        // 1. Match the pattern against component names
        // 2. Apply the template to matched components
        // 3. Return the transformed result
        
        self.logger.debug(&format!("Executing template pattern: {}", pattern));
        
        // For now, return a simple template result
        Ok(Value::String(format!("Template result for pattern: {}", pattern)))
    }
    
    /// Execute multiple components with dependency resolution
    pub fn execute_with_dependencies(
        &self,
        components: &HashMap<String, crate::models::YMXComponent>,
        root_component: &str,
        initial_context: &HashMap<String, Value>,
    ) -> Result<Value> {
        // Simple dependency resolution and execution
        // In a full implementation, this would:
        // 1. Build dependency graph
        // 2. Topologically sort components
        // 3. Execute in dependency order
        
        self.logger.debug(&format!("Executing dependency graph starting from: {}", root_component));
        
        let mut context = initial_context.clone();
        let mut executed = std::collections::HashSet::new();
        
        self.execute_dependency_graph(components, root_component, &mut context, &mut executed)
    }
    
    /// Recursive dependency graph execution
    fn execute_dependency_graph(
        &self,
        components: &HashMap<String, crate::models::YMXComponent>,
        component_name: &str,
        context: &mut HashMap<String, Value>,
        executed: &mut std::collections::HashSet<String>,
    ) -> Result<Value> {
        // Prevent circular dependencies
        if executed.contains(component_name) {
            return Err(YmxError::CircularDependency {
                cycle: vec![component_name.to_string()],
            });
        }
        
        executed.insert(component_name.to_string());
        
        // Get the component
        let component = components.get(component_name)
            .ok_or_else(|| YmxError::ComponentNotFound {
                component_id: component_name.to_string(),
            })?;
        
        // Execute dependencies first
        for dependency in &component.metadata.dependencies {
            if components.contains_key(dependency) && !executed.contains(dependency) {
                self.execute_dependency_graph(components, dependency, context, executed)?;
            }
        }
        
        // Execute the current component
        let result = self.execute_component(component, context)?;
        
        // Store result in context for dependent components
        context.insert(component_name.to_string(), result);
        
        Ok(result)
    }
    
    /// Validate execution environment against security policy
    pub fn validate_execution(&self, _env: &ExecutionEnvironment) -> Result<()> {
        // This is a simplified validation
        // In a real implementation, this would check against security policy
        Ok(())
    }
}