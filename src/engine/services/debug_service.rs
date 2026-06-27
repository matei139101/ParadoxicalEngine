pub use crate::prelude::*;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    widgets::{Block, Borders, List, ListDirection, Tabs},
    Frame, Terminal,
};

/// Handles all debug related tasks such as updating the devtool dashboard and logs.
pub struct DebugService {
    terminal: RwLock<Terminal<CrosstermBackend<io::Stdout>>>,
    selected_tab: RwLock<usize>,
}

impl DebugService {
    pub fn new() -> Self {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

        Self {
            terminal: RwLock::new(terminal),
            selected_tab: 0.into(),
        }
    }

    /// Draws the dashboard to the terminal.
    fn draw_terminal(&self) {
        self.terminal
            .write()
            .expect("Program failed to writelock the terminal for logging...")
            .draw(|frame| {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Fill(1)])
                    .split(frame.area());

                let selected_tab = self.selected_tab.read().unwrap_or_else(|_| {
                    log!(
                        Self,
                        Critical,
                        "Failed to readlock selected dashboard tab..."
                    );
                    panic!()
                });

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
            })
            .unwrap();
    }

    /// Draws the logs to the terminal.
    fn render_logs(&self, frame: &mut Frame, layout: Rect) {
        let mut logs = crate::LOGGER.get_logs();
        logs.reverse();

        frame.render_widget(
            List::new(logs)
                .direction(ListDirection::BottomToTop)
                .block(Block::default().borders(Borders::ALL).title("Logs")),
            layout,
        );
    }

    /// Draws the statistics to the terminal.
    fn render_stats(&self, _frame: &mut Frame, _layout: Rect) {}
}

impl Service for DebugService {
    fn update(&self) {
        log!(Self, Critical, "Updating debug service.");
        static LAST_TERMINAL_UPDATE: RwLock<Option<Instant>> = RwLock::new(None);

        if let Ok(last_update) = LAST_TERMINAL_UPDATE.read() {
            if let Some(last_update) = last_update.as_ref() {
                if last_update.elapsed() <= Duration::from_millis(16) {
                    return;
                }
            }
            drop(last_update);

            *LAST_TERMINAL_UPDATE.write().unwrap() = Some(Instant::now());

            self.draw_terminal();

            if event::poll(Duration::from_millis(0)).unwrap() {
                match event::read().unwrap() {
                    Event::Key(key) if key.code == KeyCode::Tab => {
                        *self.selected_tab.write().unwrap_or_else(|_| {
                            log!(
                                Self,
                                Critical,
                                "Failed to writelock selected dashboard tab..."
                            );
                            panic!()
                        }) ^= 1;
                    }
                    _ => {}
                }
            }
        } else {
            log!(Self, Critical, "Failed to readlock LAST_UPDATE...");
        }
    }

    fn get_data(&self) {}
}
