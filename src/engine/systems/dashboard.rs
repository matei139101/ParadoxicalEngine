use std::{io};

use crossterm::event::{self, Event, KeyCode};
use once_cell::sync::Lazy;
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, prelude::CrosstermBackend, widgets::{Block, Borders, List, ListDirection, Tabs}, Frame, Terminal};

use crate::prelude::*;

pub struct Dashboard {
    terminal: RwLock<Terminal<CrosstermBackend<io::Stdout>>>,
    dashboard_data: DashboardData,

    selected_tab: RwLock<usize>,
}

impl Dashboard {
    fn new() -> Self {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

        Self { terminal: RwLock::new(terminal), dashboard_data: DashboardData::new(), selected_tab: 0.into()}
    }

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

    fn render_dashboard(&self) {

    }

    fn render_logs(&self, frame: &mut Frame, layout: Rect) {
        let logs = self.dashboard_data.logs.read().unwrap_or_else(|_| {log!(Self, Critical, "Failed to readlock logs..."); panic!()}).clone();

        frame.render_widget(
            List::new(logs).direction(ListDirection::BottomToTop).block(Block::default().borders(Borders::ALL).title("Logs")),
            layout,
        );
    }

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

    pub fn dashboard_data(&self) -> &DashboardData {
        &self.dashboard_data
    }
}

pub static DASHBOARD: Lazy<Dashboard> = Lazy::new(Dashboard::new);

pub struct DashboardData {
    render_stats: RenderStats,
    logs: RwLock<Vec<String>>,
}

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
