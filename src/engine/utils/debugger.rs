use std::{
    collections::HashMap,
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

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Dev = 4,
    Critical = 5,
}

pub struct Debugger {
    logs: Mutex<Vec<String>>,
    widgets: Mutex<HashMap<String, Arc<RwLock<f32>>>>,
    widget_window_size: usize,
    log_window_size: usize,
}

impl Debugger {
    pub fn new() -> Self {
        let debugger = Self {
            logs: Mutex::new(Vec::new()),
            widgets: Mutex::new(HashMap::new()),
            widget_window_size: 5,
            log_window_size: 5,
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
        if level <= LogLevel::Dev {
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
        if level <= LogLevel::Dev {
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

    pub fn add_widget(&self, value_name: String, value_pointer: Arc<RwLock<f32>>) {
        self.widgets
            .lock()
            .unwrap()
            .insert(value_name, value_pointer);

        self.update_terminal();
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
                (self.widget_window_size + self.log_window_size + 2) as u16
            ),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        println!("{}", "─".repeat(terminal_width.into()));
        execute!(
            stdout(),
            MoveTo(
                0,
                (self.widget_window_size + self.log_window_size + 2) as u16
            ),
        )
        .unwrap();
        println!("BOTTOM-BORDER  |");
    }

    fn update_log(&self) {
        let locked_logs = self.logs.lock().unwrap();
        let total_logs = locked_logs.len();
        let start_index = total_logs.saturating_sub(self.log_window_size);

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

        let widgets = self.widgets.lock().unwrap();
        for (key, value) in widgets.iter() {
            print!("{} : {} | ", key, value.read().unwrap())
        }

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

#[macro_export]
macro_rules! widget {
    ($value_name:expr, $value_ptr:expr) => {
        $crate::DEBUGGER.add_widget($value_name, $value_ptr);
    };
}
