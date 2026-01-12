use crate::error::{SourceLocation, YmxError};
use crate::utils::logging::{get_logger, PerformanceTimer};
use std::time::Duration;

/// Enhanced error reporting with line/column information
pub struct ErrorReporter {
    logger: crate::utils::logging::Logger,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            logger: get_logger(),
        }
    }

    /// Report error with performance tracking
    pub fn report_error(&self, error: &YmxError) {
        let timer = PerformanceTimer::new("error_reporting", Duration::from_millis(10));

        let (message, location) = match error {
            YmxError::ParseError { message, location } => (message.clone(), location.clone()),
            YmxError::YamlSyntaxError {
                message,
                line,
                column,
                ..
            } => (message.clone(), SourceLocation::new("", *line, *column, 0)),
            YmxError::ComponentNotFound { component_id } => (
                format!("Component not found: {}", component_id),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::InvalidPropertyReference { property } => (
                format!("Invalid property reference: {}", property),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::ExecutionTimeout { limit } => (
                format!("Execution timeout: exceeded {}", limit),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::MemoryLimitExceeded { used, limit } => (
                format!("Memory limit exceeded: used {}, limit {}", used, limit),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::SecurityViolation { violation } => (
                format!("Security violation: {}", violation),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::InterpreterError { error } => (
                format!("Interpreter error: {}", error),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::IoError(_) => (
                "I/O error occurred".to_string(),
                SourceLocation::new("", 0, 0, 0),
            ),
            YmxError::ConfigurationError { message } => (
                format!("Configuration error: {}", message),
                SourceLocation::new("", 0, 0, 0),
            ),
        };

        let elapsed = timer.finish(&self.logger).unwrap_or_default();

        self.display_error(&message, &location, elapsed.as_millis());
    }

    /// Display error with location and formatting
    fn display_error(&self, message: &str, location: &SourceLocation, elapsed_ms: u64) {
        let (color, prefix) = self.get_error_display_info();

        if location.file.display().to_string().is_empty() {
            // Runtime error (no file location)
            eprintln!(
                "{}{}{}{}{}",
                color,
                prefix,
                message.bold(),
                if elapsed_ms > 10 {
                    format!(" {} (slow: {}ms)", "warning".yellow(), elapsed_ms)
                } else {
                    String::new()
                },
                "\n".normal()
            );
        } else {
            // File-based error
            eprintln!(
                "{}{}{}:{}:{}{}{}",
                color,
                prefix,
                location.file.display().to_string().cyan(),
                location.line.to_string().magenta(),
                location.column.to_string().yellow(),
                message.bold(),
                if elapsed_ms > 10 {
                    format!(" {} (slow: {}ms)", "warning".yellow(), elapsed_ms)
                } else {
                    String::new()
                },
                "\n".normal()
            );
        }
    }

    /// Get color and prefix based on error severity
    fn get_error_display_info(&self) -> (colored::Color, &'static str) {
        (colored::Color::Red, "ERROR: ")
    }

    /// Report multiple errors with summary
    pub fn report_errors(&self, errors: &[YmxError]) {
        if errors.is_empty() {
            return;
        }

        self.logger
            .error(&format!("Found {} error(s)", errors.len()));

        for (index, error) in errors.iter().enumerate() {
            eprintln!("{}. ", index + 1);
            self.report_error(error);
        }

        // Provide helpful suggestions
        self.suggest_fixes(errors);
    }

    /// Provide helpful suggestions based on error types
    fn suggest_fixes(&self, errors: &[YmxError]) {
        let mut suggestions = Vec::new();

        for error in errors {
            match error {
                YmxError::YamlSyntaxError { .. } => {
                    suggestions
                        .push("Check YAML syntax: ensure proper indentation and quote usage");
                }
                YmxError::InvalidPropertyReference { .. } => {
                    suggestions
                        .push("Verify property names are spelled correctly and defined in context");
                }
                YmxError::ComponentNotFound { .. } => {
                    suggestions.push(
                        "Check component name spelling and ensure component file is included",
                    );
                }
                YmxError::CircularDependency { .. } => {
                    suggestions.push("Review component dependencies to break circular references");
                }
                YmxError::ExecutionTimeout { .. } => {
                    suggestions
                        .push("Consider optimizing component logic or increasing timeout limits");
                }
                YmxError::MemoryLimitExceeded { .. } => {
                    suggestions.push("Reduce component complexity or increase memory limits");
                }
                YmxError::SecurityViolation { .. } => {
                    suggestions.push("Review security policy and adjust component permissions");
                }
                _ => {}
            }
        }

        // Remove duplicates
        suggestions.sort();
        suggestions.dedup();

        if !suggestions.is_empty() {
            println!("\n{}Suggestions:", "hint:".bright_blue());
            for suggestion in suggestions {
                println!("  • {}", suggestion);
            }
        }
    }

    /// Report warning with location
    pub fn report_warning(&self, message: &str, location: &SourceLocation) {
        let (color, prefix) = (colored::Color::Yellow, "WARNING: ");

        if location.file.display().to_string().is_empty() {
            eprintln!("{}{}{}{}", color, prefix, message.yellow(), "\n".normal());
        } else {
            eprintln!(
                "{}{}:{}:{} {}{}",
                color,
                location.file.display().to_string().cyan(),
                location.line.to_string().magenta(),
                location.column.to_string().yellow(),
                message.yellow(),
                "\n".normal()
            );
        }
    }

    /// Create formatted error output for CLI
    pub fn format_error_for_cli(&self, error: &YmxError) -> String {
        match error {
            YmxError::ParseError { message, location } => {
                format!("Parse error at {}: {}", location.display(), message)
            }
            YmxError::YamlSyntaxError {
                message,
                line,
                column,
                ..
            } => {
                format!(
                    "YAML syntax error at line {}, column {}: {}",
                    line, column, message
                )
            }
            YmxError::ComponentNotFound { component_id } => {
                format!("Component not found: {}", component_id)
            }
            YmxError::InvalidPropertyReference { property } => {
                format!("Invalid property reference: {}", property)
            }
            YmxError::ExecutionTimeout { limit } => {
                format!("Execution timeout exceeded: {}", limit)
            }
            YmxError::MemoryLimitExceeded { used, limit } => {
                format!(
                    "Memory limit exceeded: used {} bytes, limit {} bytes",
                    used, limit
                )
            }
            YmxError::SecurityViolation { violation } => {
                format!("Security violation: {}", violation)
            }
            YmxError::InterpreterError { error } => {
                format!("Interpreter error: {}", error)
            }
            YmxError::IoError(_) => "I/O error occurred".to_string(),
            YmxError::ConfigurationError { message } => {
                format!("Configuration error: {}", message)
            }
        }
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}
