use std::thread::JoinHandle;
use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::prelude::*;

/// Used for smartly handling threads and asynchronous work between engine services.
///
/// Asynchronously handling work can be heavily benificial for engine performance but difficult to
/// manage. The [`Scheduler`] exists for the sole reason of managing threads and ensuring work gets
/// distributed between the threads properly.
pub struct Scheduler {
    scheduler_thread: Option<JoinHandle<()>>,
    scheduler_channel: (Sender<i32>, Receiver<i32>)
}

impl Scheduler {
    /// Returns a new [`Scheduler`].
    pub fn new() -> Self {        
        Self {
            scheduler_thread: None,
            scheduler_channel: unbounded()
        }
    }

    /// Sets up required threads for the provided [`ServiceLocator`] and starts them.
    pub fn run(&mut self, service_locator: &ServiceLocator) {
        log!(Self, Critical, "Starting scheduler thread.");

        let channel = self.scheduler_channel.clone();
        self.scheduler_thread = Some(thread::spawn(|| {Self::thread(channel)}));

        for (_service_type, service) in service_locator.iter() {
            self.start_thread(service.clone(), self.scheduler_channel.1.clone());
        }
    }

    /// Starts a thread for the provided [`Service`].
    fn start_thread(&self, service: Arc<dyn Service>, receiver: Receiver<i32>) {
        log!(Self, Critical, "Starting thread.");
        let _handle = thread::spawn(move || loop {
            service.update();
            thread::sleep(Duration::from_millis(1));
        });
    }

    fn thread(scheduler_channel: (Sender<i32>, Receiver<i32>)) {
        loop {
            log!(Self, Critical, "Tick.");
            thread::sleep(Duration::from_secs(3));
        }
    }
}
