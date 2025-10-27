#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    //Low = 1,
    //Medium = 2,
    High = 3,
    Dev = 4,
}

pub struct Logger {}

impl Logger {
    pub fn log_with_type<T>(level: LogLevel, message: &str) {
        if level <= LogLevel::Dev {
            println!(
                "{}: [{:?}] ({}) {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                std::any::type_name::<T>(),
                message
            );
        }
    }

    pub fn log_without_type(level: LogLevel, message: &str) {
        if level <= LogLevel::Dev {
            println!(
                "{}: [{:?}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                message
            );
        }
    }
}

#[macro_export]
macro_rules! log {
    ($culprit:ty, $level:expr, $msg:expr) => {
        Logger::log_with_type::<$culprit>($level, $msg);
    };
    ($level:expr, $msg:expr) => {
        Logger::log_without_type($level, $msg);
    };
}
