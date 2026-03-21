use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use once_cell::sync::Lazy;

use crate::{engine::utils::terminal_handler::TERMINAL_HANDLER, prelude::*};
mod engine;
mod prelude;
mod resources;

fn main() {
    enable_raw_mode().unwrap();
    execute!(std::io::stdout(), EnterAlternateScreen).unwrap();

    let (async_sender, async_receiver) = mpsc::channel::<Box<dyn Any + Send + Sync>>();

    let event_bus = EventBus::new();

    let repositories: Arc<Repositories> = Arc::new(Repositories::new());
    let services: Arc<Services> = Arc::new(Services::new(
        repositories,
        event_bus.clone(),
        async_sender.clone(),
    ));
    let app = make_app(async_sender.clone(), services.clone());

    let synchronizer = Synchronizer::new(services);
    let _synchronizer_handle = synchronizer.start();
    start_event_bus_thread(event_bus, async_receiver);
    start_window_thread(app);

    disable_raw_mode().unwrap();
    execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
}

fn make_app(
    async_sender: Sender<Box<dyn Any + Send + Sync>>,
    services: Arc<Services>,
) -> (App, EventLoop<()>) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let app = App::new(services.clone(), async_sender);
    (app, event_loop)
}

fn start_event_bus_thread(
    event_bus_ptr: Arc<EventBus>,
    async_receiver: Receiver<Box<dyn Any + Send + Sync>>,
) {
    thread::spawn(move || {
        log!(High, "Starting async runtime"); 
        EventBus::run(event_bus_ptr.clone(), async_receiver);
    });
}

fn start_window_thread(mut app: (App, EventLoop<()>)) {
    let _ = app.1.run_app(&mut app.0);
}
