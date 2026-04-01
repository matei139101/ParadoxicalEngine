use std::{
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;
use crate::{engine::utils::tracked_values::TrackedValues, prelude::{dashboard::TerminalData, *}};

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

pub struct Logger {
    logs: Arc<RwLock<Vec<String>>>,
    tracked_values: Arc<TrackedValues>,

    debug_level: LogLevel,
}

impl Logger {
    pub fn new() -> Self {
        let debug_level = LogLevel::from_string(FILE_HANDLER.get_config().get("Debug", "debuglevel").unwrap());
        Logger {
            logs: Arc::new(RwLock::new(Vec::new())),
            tracked_values: Arc::new(TrackedValues::new()),

            debug_level,
        }
    }

    pub fn update(&self) {
        static LAST_LOGGER_UPDATE: RwLock<Option<Instant>> = RwLock::new(None);

        if let Ok(last_update) = LAST_LOGGER_UPDATE.read() {
            if let Some(last_update) = last_update.as_ref() {
                if last_update.elapsed() <= Duration::from_secs(1) {
                    return;
                }
            }
            drop(last_update);
        
            *LAST_LOGGER_UPDATE.write().unwrap() = Some(Instant::now());
            self.send_to_terminal();
        } else {
            log!(Self, Critical, "Failed to readlock LAST_UPDATE...");
        }
    }

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

    fn send_to_terminal(&self) {
        let widgets = vec![format!("fps: {}", self.tracked_values.get_fps()).to_string(), format!("frametime: {}", self.tracked_values.get_frametime()).to_string(), format!("frames: {}", self.tracked_values.get_total_frames()).to_string()];
        let mut logs = self.logs.read().unwrap().clone();
        logs.reverse();

        DASHBOARD.write(TerminalData { widgets, logs });
    }
}

pub static LOGGER: Lazy<Logger> = Lazy::new(Logger::new);

#[macro_export]
macro_rules! log {
    ($culprit:ty, $level:expr, $msg:expr) => {
        $crate::LOGGER.log_with_type::<$culprit>($level, $msg);
    };
    ($level:expr, $msg:expr) => {
        $crate::LOGGER.log_without_type($level, $msg);
    };
}
