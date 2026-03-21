use std::{io, thread::sleep};

use once_cell::sync::Lazy;
use ratatui::{layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, widgets::{self, Block, Borders, List, ListDirection, Tabs}, Frame, Terminal};

use crate::prelude::*;

#[derive(Default)]
pub struct TerminalData {
    pub widgets: Vec<String>,
    pub logs: Vec<String>,
}

pub struct TerminalHandler {
    terminal: RwLock<Terminal<CrosstermBackend<io::Stdout>>>,
    terminal_data: RwLock<TerminalData>,
}

impl TerminalHandler {
    fn new() -> TerminalHandler {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

        let terminal_handler = TerminalHandler { terminal: RwLock::new(terminal), terminal_data: Default::default()};

        thread::spawn(move || {
            let update_interval = Duration::from_secs(5);
            
            loop {
                TERMINAL_HANDLER.update(&update_interval);
            }
            });

        terminal_handler
    }

    fn update(&self, update_interval: &Duration) {
        self.draw_terminal();

        sleep(*update_interval);
    }

    fn draw_terminal(&self) {
        let widgets = self.terminal_data.read().unwrap().widgets.clone();
        let logs = self.terminal_data.read().unwrap().logs.clone();

        self.terminal.write().expect("Program failed to writelock the terminal for logging...").draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .split(frame.area());

            frame.render_widget(
                Tabs::new(widgets).block(Block::default().borders(Borders::ALL).title("Stats")),
                chunks[0],
            );

            frame.render_widget(
                List::new(logs).direction(ListDirection::BottomToTop).block(Block::default().borders(Borders::ALL).title("Logs")),
                chunks[1],
            );
        }).unwrap();
    }

    pub fn write(&self, data: TerminalData) {
        *self.terminal_data.write().unwrap() = data;
    }
}

pub static TERMINAL_HANDLER: Lazy<TerminalHandler> = Lazy::new(TerminalHandler::new);
