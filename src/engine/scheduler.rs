use crate::prelude::*;

/// Used for smartly handling threads and asynchronous work between engine services.
///
/// Asynchronously handling work can be heavily benificial for engine performance but difficult to
/// manage. The [`Scheduler`] exists for the sole reason of managing threads and ensuring work gets
/// distributed between the threads properly.
pub struct Scheduler {}

impl Scheduler {
    /// Returns a new [`Scheduler`].
    pub fn new() -> Self {
        Self {}
    }

    /// Sets up required threads for the provided [`ServiceLocator`] and starts them.
    pub fn run(&self, service_locator: &ServiceLocator) {
        let _ = service_locator.iter().map(|(_service_type, service)| {
            self.start_thread(service.clone());
        });
    }

    /// Starts a thread for the provided [`Service`].
    fn start_thread(&self, service: Arc<dyn Service>) {
        log!(Self, Critical, "Starting thread.");
        let _ = thread::spawn(move || loop {
            service.update();
        });
    }

    /// Starts the [`Window`] event loop on the main thread
    fn start_window_thread(&self, event_loop: EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = Window::new();

        let _ = event_loop.run_app(&mut app);
    }
}
