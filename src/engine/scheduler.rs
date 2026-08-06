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
    pub fn run(&mut self, service_locator: Arc<ServiceLocator>) {
        log!(Self, Critical, "Starting scheduler thread.");

        self.scheduler_thread = Some(thread::spawn(|| Self::thread(service_locator)));
    }

    fn thread(service_locator: Arc<ServiceLocator>) {
        loop {
            log!(Self, Critical, "Update...");
            service_locator.iter().for_each(|(_service_type, service)| {
                service.update();
            });

            thread::sleep(Duration::from_secs(1));
        }
    }
}
