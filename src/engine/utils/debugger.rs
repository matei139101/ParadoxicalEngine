use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{self, Clear, ClearType},
};
use once_cell::sync::Lazy;
use std::io::{stdout, Write};
use tokio::runtime::Runtime;
use crate::{engine::utils::tracked_values::TrackedValues, prelude::*};

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
    logs: Mutex<Vec<String>>,
    tracked_values: TrackedValues,

    widget_window_size: usize,
    debug_lines: usize,
    debug_level: LogLevel,
}

impl Debugger {
    pub fn new() -> Self {
        let debug_lines = FILE_HANDLER.get_config().getuint("Debug", "debuglines").unwrap().unwrap() as usize;
        let debug_level = LogLevel::from_string(FILE_HANDLER.get_config().get("Debug", "debuglevel").unwrap());
        let debugger = Self {
            logs: Mutex::new(Vec::new()),
            tracked_values: TrackedValues::new(),

            widget_window_size: 5,
            debug_lines,
            debug_level,
        };

        debugger.create_terminal_window();
        Debugger::start_debugger();

        debugger
    }

    fn start_debugger() {
        std::thread::spawn(|| {
            let debugger_runtime = Runtime::new().unwrap();

            debugger_runtime.block_on(async {
                loop {
                    DEBUGGER.update_terminal();
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });
        });
    }

    pub fn log_with_type<T>(&self, level: LogLevel, message: &str) {
        if level <= self.debug_level {
            let formatted_message: String = format!(
                "{}: [{:?}] ({}) {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                std::any::type_name::<T>(),
                message
            );

            self.logs.lock().unwrap().push(formatted_message);
            self.update_terminal();
        }
    }

    pub fn log_without_type(&self, level: LogLevel, message: &str) {
        if level <= self.debug_level {
            let formatted_message: String = format!(
                "{}: [{:?}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                message
            );

            self.logs.lock().unwrap().push(formatted_message);
            self.update_terminal();
        }
    }

    fn create_terminal_window(&self) {
        let (terminal_width, _) = terminal::size().expect("No terminal was found?");
        execute!(stdout(), Clear(ClearType::All)).unwrap();

        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::CurrentLine)).unwrap();
        println!("{}", "─".repeat(terminal_width.into()));
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        println!("TOP-BORDER     |");

        execute!(
            stdout(),
            MoveTo(0, (self.widget_window_size + 1) as u16),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        println!("{}", "─".repeat(terminal_width.into()));
        execute!(stdout(), MoveTo(0, (self.widget_window_size + 1) as u16)).unwrap();
        println!("SEPERATOR      |");

        execute!(
            stdout(),
            MoveTo(
                0,
                (self.widget_window_size + self.debug_lines + 2) as u16
            ),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        println!("{}", "─".repeat(terminal_width.into()));
        execute!(
            stdout(),
            MoveTo(
                0,
                (self.widget_window_size + self.debug_lines + 2) as u16
            ),
        )
        .unwrap();
        println!("BOTTOM-BORDER  |");
    }

    fn update_log(&self) {
        let locked_logs = self.logs.lock().unwrap();
        let total_logs = locked_logs.len();
        let start_index = total_logs.saturating_sub(self.debug_lines);

        for (i, message) in locked_logs.iter().skip(start_index).enumerate() {
            let row = (i + self.widget_window_size + 2) as u16;
            execute!(stdout(), MoveTo(0, row), Clear(ClearType::CurrentLine)).unwrap();
            print!("{}", message);
        }

        stdout().flush().unwrap();
        self.move_cursor_to_bottom();
    }

    fn update_widgets(&self) {
        execute!(stdout(), MoveTo(0, 3),).unwrap();

        print!("fps : {} | ", self.tracked_values.get_fps());
        print!("frametime : {} | ", self.tracked_values.get_frametime() as f32 / 1000.0);
        print!("total frames : {} | ", self.tracked_values.get_total_frames());

        self.move_cursor_to_bottom();
    }

    fn move_cursor_to_bottom(&self) {
        let _ = execute!(
            stdout(),
            MoveTo(0, terminal::size().expect("No terminal was found?").1),
        );
    }

    fn update_terminal(&self) {
        self.create_terminal_window();
        self.update_widgets();
        self.update_log();
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
