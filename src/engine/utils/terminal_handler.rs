use std::{io};

use once_cell::sync::Lazy;
use ratatui::{layout::{Constraint, Direction, Layout}, prelude::CrosstermBackend, widgets::{Block, Borders, List, ListDirection, Tabs}, Terminal};

use crate::prelude::*;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Default)]
pub struct TerminalData {
    pub widgets: Vec<String>,
    pub logs: Vec<String>,
}

pub struct TerminalHandler {
    terminal: RwLock<Terminal<CrosstermBackend<io::Stdout>>>,
    terminal_data: RwLock<TerminalData>,
    sender: mpsc::Sender<Job>,
}

impl TerminalHandler {
    fn new() -> TerminalHandler {
        let (sender, receiver) = mpsc::channel::<Job>();
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();
        let terminal_handler = TerminalHandler { terminal: RwLock::new(terminal), terminal_data: Default::default(), sender};

        let _ = thread::Builder::new().name("Terminal".to_string()).spawn(move || {
            while let Ok(job) = receiver.recv() {
                job();
            }
        });

        terminal_handler
    }

    fn run(&self, f: impl FnOnce() + Send + 'static) {
        self.sender.send(Box::new(f)).unwrap();
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
        self.run(move || {
            *TERMINAL_HANDLER.terminal_data.write().expect("Program failed to obtain terminal data...") = data;
            TERMINAL_HANDLER.draw_terminal();
        });
    }
}

pub static TERMINAL_HANDLER: Lazy<TerminalHandler> = Lazy::new(TerminalHandler::new);
