use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{prelude::*};
mod engine;
mod prelude;
mod resources;

fn main() {
    enable_raw_mode().unwrap();
    execute!(std::io::stdout(), EnterAlternateScreen).unwrap();

    let (async_sender, async_receiver) = mpsc::channel::<Box<dyn Any + Send + Sync>>();

    let event_bus = Arc::new(EventBus::new());

    let repositories: Arc<Repositories> = Arc::new(Repositories::new());
    let services: Arc<Services> = Arc::new(Services::new(
        repositories,
        event_bus.clone(),
        async_sender.clone(),
    ));

    let synchronizer = Scheduler::new(async_sender, services, event_bus);
    synchronizer.start(async_receiver);

    disable_raw_mode().unwrap();
    execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
}
