use crossterm::event::{self, Event, KeyCode};
use once_cell::sync::Lazy;
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, widgets::{Block, Borders, List, ListDirection, Tabs}, Frame, Terminal};

use crate::prelude::*;

/// Defines the dashboard system
/// 
/// The dashboard system contains all logic directly related to rendering the dashboard in the
/// terminal. All data to be displayed is stored in the [`DashboardData`] struct and rendered to
/// the terminal once an update is called.
pub struct Dashboard {
    terminal: RwLock<Terminal<CrosstermBackend<io::Stdout>>>,
    dashboard_data: DashboardData,

    selected_tab: RwLock<usize>,
}

impl Dashboard {
    /// Returns a new Dashboard created using default data.
    fn new() -> Self {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

        Self { terminal: RwLock::new(terminal), dashboard_data: DashboardData::new(), selected_tab: 0.into()}
    }

    /// Handles whether the terminal should be drawn to and listens to input.
    pub fn update(&self) {
        static LAST_TERMINAL_UPDATE: RwLock<Option<Instant>> = RwLock::new(None);

        if let Ok(last_update) = LAST_TERMINAL_UPDATE.read() {
            if let Some(last_update) = last_update.as_ref() {
                if last_update.elapsed() <= Duration::from_millis(16){
                    return;
                }
            }
            drop(last_update);
        
            *LAST_TERMINAL_UPDATE.write().unwrap() = Some(Instant::now());

            self.draw_terminal();

            if event::poll(Duration::from_millis(0)).unwrap() {
                match event::read().unwrap() {
                    Event::Key(key) if key.code == KeyCode::Tab => {
                        *self.selected_tab.write().unwrap_or_else(|_| {log!(Self, Critical, "Failed to writelock selected dashboard tab..."); panic!()}) ^= 1;
                    }
                    _ => {}
                }
            }
        } else {
            log!(Self, Critical, "Failed to readlock LAST_UPDATE...");
        }
    }

    /// Draws the dashboard to the terminal.
    fn draw_terminal(&self) {
        self.terminal.write().expect("Program failed to writelock the terminal for logging...").draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .split(frame.area());

            let selected_tab = self.selected_tab.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock selected dashboard tab..."); panic!()});

            frame.render_widget(
                Tabs::new(vec!["Logs", "Stats"])
                .select(*selected_tab)
                .block(Block::default().borders(Borders::ALL)),
                layout[0],
            );

            match *selected_tab {
                0 => self.render_logs(frame, layout[1]),
                1 => self.render_stats(frame, layout[1]),
                _ => {}
            }
            
        }).unwrap();
    }

    /// Draws the logs to the terminal.
    fn render_logs(&self, frame: &mut Frame, layout: Rect) {
        let logs = self.dashboard_data.logs.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock logs..."); panic!()}).clone();

        frame.render_widget(
            List::new(logs).direction(ListDirection::BottomToTop).block(Block::default().borders(Borders::ALL).title("Logs")),
            layout,
        );
    }

    /// Draws the statistics to the terminal.
    fn render_stats(&self, frame: &mut Frame, layout: Rect) {
        let fps = self.dashboard_data.render_stats.fps.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock fps stat..."); panic!()}).clone();
        let frametime = self.dashboard_data.render_stats.frametime.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock frametime stat..."); panic!()}).clone();
        let frames = self.dashboard_data.render_stats.frames.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock frames stat..."); panic!()}).clone();
        let data = [fps, frametime, frames];

        frame.render_widget(
            List::new(data).direction(ListDirection::TopToBottom).block(Block::default().borders(Borders::ALL).title("Render Data")),
            layout,
        );
    }

    /// Returns a reference to dashboard data for reading or editing purposes. Any changes will be
    /// displayed on the next update.
    pub fn dashboard_data(&self) -> &DashboardData {
        &self.dashboard_data
    }
}

pub struct DashboardData {
    render_stats: RenderStats,
    logs: RwLock<Vec<String>>,
}

/// Defines the struct containing all data rendered to the [`Dashboard`].
impl DashboardData {
    pub fn new() -> Self {
        Self { render_stats: RenderStats::new(), logs: Vec::new().into() }
    }

    pub fn render_stats(&self) -> &RenderStats {
        &self.render_stats
    }

    pub fn logs(&self) -> &RwLock<Vec<String>> {
        &self.logs
    }
}

/// Defines the struct containing all render statictics used by [`DashboardData`].
pub struct RenderStats {
    fps: RwLock<String>,
    frametime: RwLock<String>,
    frames: RwLock<String>,
}

impl RenderStats {
    pub fn new() -> Self {
        Self { fps: Default::default(), frametime: Default::default(), frames: Default::default() }
    }

    pub fn fps(&self) -> &RwLock<String> {
        &self.fps
    }

    pub fn frametime(&self) -> &RwLock<String> {
        &self.frametime
    }

    pub fn frames(&self) -> &RwLock<String> {
        &self.frames
    }
}

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

    /// Periodically sends the data from the logger to the dashboard to be rendered in the
    /// terminal.
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
        } else {
            log!(Self, Critical, "Failed to readlock LAST_UPDATE...");
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
