use crate::error::{Result, YmxError};
use crate::interpreters::{create_interpreter, InterpreterEngine};
use crate::models::{ExecutionEnvironment, Value};
use crate::utils::logging::{get_logger, PerformanceTimer};
use std::time::Duration;

/// Simple logging configuration for YMX
pub struct Logger {
    enabled: bool,
    level: LogLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl Logger {
    pub fn new(enabled: bool, level: LogLevel) -> Self {
        Self { enabled, level }
    }

    pub fn error(&self, message: &str) {
        if self.enabled && self.should_log(LogLevel::Error) {
            eprintln!("{}{}{}", "ERROR: ".red(), message, "\n".red());
        }
    }

    pub fn warn(&self, message: &str) {
        if self.enabled && self.should_log(LogLevel::Warn) {
            eprintln!("{}{}{}", "WARN: ".yellow(), message, "\n".yellow());
        }
    }

    pub fn info(&self, message: &str) {
        if self.enabled && self.should_log(LogLevel::Info) {
            println!("{}{}", message, "\n".dimmed());
        }
    }

    pub fn debug(&self, message: &str) {
        if self.enabled && self.should_log(LogLevel::Debug) {
            println!("{}{}", "DEBUG: ".bright_blue(), message, "\n".bright_blue());
        }
    }

    fn should_log(&self, message_level: LogLevel) -> bool {
        match (message_level, &self.level) {
            (LogLevel::Error, _) => true,
            (LogLevel::Warn, LogLevel::Error | LogLevel::Warn) => false,
            (LogLevel::Info, LogLevel::Error | LogLevel::Warn | LogLevel::Info) => false,
            (LogLevel::Debug, _) => true,
            (a, b) => a <= b,
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(true, LogLevel::Info)
    }
}

/// Performance timer for measuring execution times
pub struct PerformanceTimer {
    start_time: std::time::Instant,
    operation: String,
    timeout: Duration,
}

impl PerformanceTimer {
    pub fn new(operation: &str, timeout: Duration) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            operation: operation.to_string(),
            timeout,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn check_timeout(&self) -> Result<(), YmxError> {
        let elapsed = self.elapsed();
        if elapsed > self.timeout {
            Err(YmxError::InterpreterError {
                error: format!("Operation timed out after {:?}", elapsed),
            })
        } else {
            Ok(())
        }
    }

    pub fn finish(self, logger: &Logger) -> Result<Duration> {
        let elapsed = self.elapsed();

        // Check if we exceeded performance requirements
        if self.operation == "parsing" && elapsed.as_millis() > performance::PARSE_TIME_LIMIT_MS {
            logger.warn(&format!(
                "Parsing exceeded performance requirement: {}ms > {}ms",
                elapsed.as_millis(),
                performance::PARSE_TIME_LIMIT_MS
            ));
        }

        if self.operation == "error_reporting"
            && elapsed.as_millis() > performance::ERROR_REPORTING_TIME_MS
        {
            logger.warn(&format!(
                "Error reporting exceeded performance requirement: {}ms > {}ms",
                elapsed.as_millis(),
                performance::ERROR_REPORTING_TIME_MS
            ));
        }

        logger.debug(&format!(
            "{} completed in {}ms",
            self.operation,
            elapsed.as_millis()
        ));
        Ok(elapsed)
    }
}

/// Memory usage tracker
pub struct MemoryTracker {
    initial_memory: usize,
    limit: usize,
}

impl MemoryTracker {
    pub fn new(limit: usize) -> Self {
        let initial_memory = Self::get_current_memory();
        Self {
            initial_memory,
            limit,
        }
    }

    pub fn current_usage(&self) -> usize {
        Self::get_current_memory() - self.initial_memory
    }

    pub fn check_limit(&self) -> Result<(), YmxError> {
        let usage = self.current_usage();
        if usage > self.limit {
            Err(YmxError::InterpreterError {
                error: format!(
                    "Memory limit exceeded: used {}, limit {}",
                    usage, self.limit
                ),
            })
        } else {
            Ok(())
        }
    }

    fn get_current_memory() -> usize {
        // Simple memory tracking - in real implementation,
        // this would use platform-specific APIs
        0 // Placeholder
    }
}

/// Global logger instance
static mut GLOBAL_LOGGER: Option<Logger> = None;

/// Initialize the global logger
pub fn init_logger(enabled: bool, level: LogLevel) {
    unsafe {
        GLOBAL_LOGGER = Some(Logger::new(enabled, level));
    }
}

/// Get the global logger
pub fn get_logger() -> &'static Logger {
    unsafe { GLOBAL_LOGGER.as_ref().unwrap_or(&Logger::default()) }
}

/// Log macros for convenience
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::utils::logging::get_logger().error(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::utils::logging::get_logger().warn(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::utils::logging::get_logger().info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::utils::logging::get_logger().debug(&format!($($arg)*));
    };
}
