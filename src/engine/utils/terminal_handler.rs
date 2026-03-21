use once_cell::sync::Lazy;

use crate::prelude::*;

enum InfoType {
    Statistic,
    Log,
}

struct TerminalMessage {
    info_type: InfoType,
    message: Vec<String>,
}

struct TerminalHandler {
    display_sender: Sender<TerminalMessage>
}

impl TerminalHandler {
    fn new() -> TerminalHandler {
        let (display_sender, display_receiver) = mpsc::channel::<TerminalMessage>();
        let terminal_handler = TerminalHandler { display_sender };

        thread::spawn(move || {
            let mut last_periodic = Instant::now();
            let update_interval = Duration::from_secs(5);
            
            loop {
                TERMINAL_HANDLER.update(&display_receiver, &mut last_periodic, &update_interval);
            }
            });

        terminal_handler
    }

    fn update(&self, display_receiver: &Receiver<TerminalMessage>, last_periodic: &mut Instant, update_interval: &Duration) {
        match display_receiver.recv_timeout(*update_interval) {
            Ok(msg) => { /* handle message */ }
            Err(mpsc::RecvTimeoutError::Timeout) => { /* periodic update */ }
            Err(mpsc::RecvTimeoutError::Disconnected) => { /* channel closed */ }
        }
    }
}

pub static TERMINAL_HANDLER: Lazy<TerminalHandler> = Lazy::new(TerminalHandler::new);
