use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use once_cell::sync::Lazy;
use crate::{engine::utils::{tracked_values::TrackedValues}, prelude::*};

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

            _ => LogLevel::Low
        }
    }
}

pub struct Debugger {
    logs: Arc<RwLock<Vec<String>>>,
    tracked_values: Arc<TrackedValues>,

    debug_level: LogLevel,
}

impl Debugger {
    pub fn new() -> Self {
        let debug_level = LogLevel::from_string(FILE_HANDLER.get_config().get("Debug", "debuglevel").unwrap());
        let debugger = Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            tracked_values: Arc::new(TrackedValues::new()),

            debug_level,
        };
        debugger.start_debugger();

        debugger
    }

    fn start_debugger(&self) {
        /*
        let tracked_values = self.tracked_values.clone();
        let logs = self.logs.clone();

        std::thread::spawn(move || {
            let debugger_runtime = Runtime::new().unwrap();

            debugger_runtime.block_on(async {
                loop {
                    TERMINAL_HANDLER.create_terminal_window();
                    TERMINAL_HANDLER.write(LoggableType::Widget(tracked_values.clone()));
                    TERMINAL_HANDLER.write(LoggableType::Log(logs.clone()));
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });
        });
        */
    }

    pub fn log_with_type<T>(&self, level: LogLevel, message: &str) {
        /*
        if level >= self.debug_level {
            let formatted_message: String = format!(
                "{}: [{:?}] ({}) {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                std::any::type_name::<T>(),
                message
            );

            self.logs.write().unwrap().push(formatted_message);
            TERMINAL_HANDLER.write(LoggableType::Log(self.logs.clone()));
        }
        */
    }

    pub fn log_without_type(&self, level: LogLevel, message: &str) {
        /*
        if level >= self.debug_level {
            let formatted_message: String = format!(
                "{}: [{:?}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                message
            );

            self.logs.write().unwrap().push(formatted_message);
            TERMINAL_HANDLER.write(LoggableType::Log(self.logs.clone()));
        }
        */
    }

    pub fn new_frame(&self) {
        let last_frame = self.tracked_values.get_last_frame();
        let new_frame = Instant::now();
        let frametime = new_frame - last_frame;
        let fps = 1.0 / frametime.as_secs_f32();
        let total_frames = self.tracked_values.get_total_frames() + 1;
        
        self.tracked_values.set_total_frames(total_frames);
        self.tracked_values.set_last_frame(new_frame);
        self.tracked_values.set_frametime(frametime.as_micros());
        self.tracked_values.set_fps(fps);
    }
}

pub static DEBUGGER: Lazy<Debugger> = Lazy::new(Debugger::new);

#[macro_export]
macro_rules! log {
    ($culprit:ty, $level:expr, $msg:expr) => {
        $crate::DEBUGGER.log_with_type::<$culprit>($level, $msg);
    };
    ($level:expr, $msg:expr) => {
        $crate::DEBUGGER.log_without_type($level, $msg);
    };
}
