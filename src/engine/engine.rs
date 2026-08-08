use crate::prelude::*;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::panic;

/// The main struct containing top level engine variables and functions.
///
/// Main purpose is to clean up main and give structure to engine setup, shutdown and other top
/// level functions.
pub struct Engine {
    service_locator: Arc<ServiceLocator>,
    scheduler: Scheduler,
}

impl Engine {
    /// Returns a new [`Engine`] containing a freshly created [`Scheduler`] and [`ServiceLocator`].
    pub fn new() -> Self {
        let debug_service = DebugService::new();
        let render_service = RenderService::new();
        let service_locator = ServiceLocator::new(render_service, debug_service);

        Self {
            service_locator: Arc::new(service_locator),
            scheduler: Scheduler::new(),
        }
    }

    /// Sets up the [`Engine`] for further use by setting hooks, preparing the terminal, and
    /// attaching services.
    pub fn setup(&mut self) {
        log!(Self, Critical, "Setting up engine runtime.");
        self.set_panic_hooks();
        enable_raw_mode().unwrap();
        execute!(std::io::stdout(), EnterAlternateScreen).unwrap();
    }

    /// Runs the [`Engine`]. This is the main loop of the engine, if this exits, this means the
    /// process is meant to be stopped.
    pub fn run(&mut self) {
        log!(Self, Critical, "Running engine runtime.");
        self.scheduler.run(self.service_locator.clone());

        log!(Self, Critical, "Starting window event loop.");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut app = Window::new(
            self.service_locator.clone(),
            self.scheduler.scheduler_sender.clone(),
        );
        let _ = event_loop.run_app(&mut app);
    }

    /// Acts as [`Engine`] clean-up.
    pub fn stop(&self) {
        log!(Self, Critical, "Stopping engine runtime.");
        disable_raw_mode().unwrap();
        execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    }

    /// Adds necessary panic hooks for general [`Engine`] functionality.
    fn set_panic_hooks(&self) {
        log!(Self, Critical, "Setting up engine panic hooks.");
        let default_hook = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen);

            default_hook(info);
        }));
    }
}
