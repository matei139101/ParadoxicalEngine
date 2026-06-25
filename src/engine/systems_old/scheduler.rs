use crate::prelude::*;
use std::thread::sleep;

/// Defines the scheduler system.
///
/// The scheduler handles creating threads and making update calls. Many engine systems/services
/// block the current thread for functionality while others simply benefit greatly from
/// asynchronous work.
///
/// Having a central scheduler ensures it is solely responsible for threading
/// and allows various tasks to be combined or split into different threads for efficient thread
/// use.
///
/// Currently doesn't contain proper thread crash handling, so threads may crash silently.
pub struct Scheduler {
    sender: Sender<Box<dyn Any + Send + Sync>>,
    services: Arc<Services>,
    event_bus: Arc<EventBus>,
}

impl Scheduler {
    /// Creates a new scheduler using a provided channel, services and event bus.
    pub fn new(
        sender: Sender<Box<dyn Any + Send + Sync>>,
        services: Arc<Services>,
        event_bus: Arc<EventBus>,
    ) -> Scheduler {
        Scheduler {
            sender,
            services,
            event_bus,
        }
    }

    /// Starts the various threads.
    pub fn start(&self, async_receiver: Receiver<Box<dyn Any + Send + Sync>>) {
        self.make_event_bus_thread(async_receiver);
        self.make_auxilliary_thread();
        self.make_service_thread();
        self.make_window_thread();
        self.make_window_thread();
    }

    /// Creates a thread used by services, checks if all services are ready for use, and periodically makes an update call. The update call
    /// automatically follows each frame due to the vulkan service update method only returning once
    /// the frame has been rendered.
    fn make_service_thread(&self) {
        log!(Self, Critical, "Starting service thread.");
        let services = Arc::clone(&self.services);

        let _ = thread::Builder::new()
            .name("Synchronizer".to_string())
            .spawn(move || {
                while !services.get_vulkan_service().is_ready() {
                    sleep(Duration::from_millis(10));
                }

                loop {
                    services.get_vulkan_service().update(&services);
                    services.get_entity_service().update(&services);
                }
            });
    }

    /// Creates a thread used by auxilliary engine components and periodically makes update calls.
    /// Used by components which benefit from constant updating and keeps the thread hot.
    fn make_auxilliary_thread(&self) {
        log!(Self, Critical, "Starting auxilliary thread.");
        let _ = thread::Builder::new()
            .name("Auxilliary".to_string())
            .spawn(move || loop {
                LOGGER.update();
                DASHBOARD.update();
            });
    }

    /// Creates a thread used exclusively by the event bus. The event bus loop is thread blocking
    /// while waiting for events and also runs all handlers on its own thread.
    fn make_event_bus_thread(&self, receiver: Receiver<Box<dyn Any + Send + Sync>>) {
        log!(Self, Dev, "Starting eventbus thread.");
        let event_bus = Arc::clone(&self.event_bus);

        let _ = thread::Builder::new()
            .name("EventBus".to_string())
            .spawn(move || {
                event_bus.run(receiver);
            });
    }

    /// Runs the main window loop. Does not create a new thread for the window as winnit doesn't
    /// support running event loops on seperate threads. Thus this method is thread blocking and
    /// blocks the main thread.
    fn make_window_thread(&self) {
        log!(Self, Dev, "Starting window thread.");

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = App::new(self.services.clone(), self.sender.clone());

        let _ = event_loop.run_app(&mut app);
    }
}
