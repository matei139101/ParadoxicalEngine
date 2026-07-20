use crate::prelude::*;
use once_cell::sync::Lazy;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Dev = 4,
    Critical = 5,
}

impl LogLevel {
    pub fn from_string(string: String) -> LogLevel {
        match string.to_uppercase().as_str() {
            "LOW" => LogLevel::Low,
            "MEDIUM" => LogLevel::Medium,
            "HIGH" => LogLevel::High,
            "DEV" => LogLevel::Dev,
            "CRITICAL" => LogLevel::Critical,

            _ => LogLevel::Low,
        }
    }
}

/// Defines the logger system.
///
/// The logger handles the various debug logging and statistics needed for development, performance
/// tracking and crashlogging.
pub struct Logger {
    logs: Arc<RwLock<Vec<String>>>,

    debug_level: LogLevel,
}

impl Logger {
    /// Returns a new logger created using a config file and default values.
    pub fn new() -> Self {
        let debug_level = LogLevel::from_string("CRITICAL".to_string());
        Logger {
            logs: Arc::new(RwLock::new(Vec::new())),

            debug_level,
        }
    }

    /// Handles log calls containing the type of the caller.
    pub fn log_with_type<T>(&self, level: LogLevel, message: &str) {
        if level >= self.debug_level {
            let formatted_message: String = format!(
                "{}: [{:?}] ({}) {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                std::any::type_name::<T>(),
                message
            );

            self.logs.write().unwrap().push(formatted_message);
        }
    }

    /// Handles log calls not containing the type of the caller.
    pub fn log_without_type(&self, level: LogLevel, message: &str) {
        if level >= self.debug_level {
            let formatted_message: String = format!(
                "{}: [{:?}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                message
            );

            self.logs.write().unwrap().push(formatted_message);
        }
    }

    /// Returns a cloned vector containing all current logs.
    pub fn get_logs(&self) -> Vec<String> {
        if let Ok(logs) = self.logs.as_ref().read() {
            logs.to_vec()
        } else {
            log!(Self, Critical, "Failed to readlock logs...");
            panic!();
        }
    }
}

pub static LOGGER: Lazy<Logger> = Lazy::new(Logger::new);

/// A macro which unifies logging with and without type into one for ease of use and handles
/// appropriately.
#[macro_export]
macro_rules! log {
    ($culprit:ty, $level:expr, $msg:expr) => {
        $crate::LOGGER.log_with_type::<$culprit>($level, $msg);
    };
    ($level:expr, $msg:expr) => {
        $crate::LOGGER.log_without_type($level, $msg);
    };
}

/// A macro which unifies panic handling in a safe and friendly way.
#[macro_export]
macro_rules! crash {
    ($culprit:ty, $level:expr, $msg:expr) => {
        $crate::LOGGER.log_with_type::<$culprit>($level, $msg);
        panic!();
    };
    ($level:expr, $msg:expr) => {
        $crate::LOGGER.log_without_type($level, $msg);
        panic!()
    };
}
