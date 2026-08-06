use std::thread::JoinHandle;

use crate::prelude::*;

/// Used for smartly handling threads and asynchronous work between engine services.
///
/// Asynchronously handling work can be heavily benificial for engine performance but difficult to
/// manage. The [`Scheduler`] exists for the sole reason of managing threads and ensuring work gets
/// distributed between the threads properly.
pub struct Scheduler {
    scheduler_thread: Option<JoinHandle<()>>,
}

impl Scheduler {
    /// Returns a new [`Scheduler`].
    pub fn new() -> Self {        
        Self {
            scheduler_thread: None,
        }
    }

    /// Sets up required threads for the provided [`ServiceLocator`] and starts them.
    pub fn run(&mut self, service_locator: &ServiceLocator) {
        log!(Self, Critical, "Starting scheduler thread.");

        self.scheduler_thread = Some(thread::spawn(|| {Self::thread()}));

        for (_service_type, service) in service_locator.iter() {
            self.start_thread(service.clone());
        }
    }

    /// Starts a thread for the provided [`Service`].
    fn start_thread(&self, service: Arc<dyn Service>) {
        log!(Self, Critical, "Starting thread.");
        let _handle = thread::spawn(move || loop {
            service.update();
            thread::sleep(Duration::from_millis(1));
        });
    }

    fn thread() {
        loop {
            log!(Self, Critical, "Tick.");
            thread::sleep(Duration::from_secs(3));
        }
    }
}
