use crossbeam_channel::{bounded, Receiver, Sender};
use std::thread::JoinHandle;

use crate::prelude::*;

/// Used for smartly handling threads and asynchronous work between engine services.
///
/// Asynchronously handling work can be heavily benificial for engine performance but difficult to
/// manage. The [`Scheduler`] exists for the sole reason of managing threads and ensuring work gets
/// distributed between the threads properly.
pub struct Scheduler {
    scheduler_thread: Option<JoinHandle<()>>,
    scheduler_receiver: Receiver<WindowReady>,
    pub scheduler_sender: Sender<WindowReady>,
}

impl Scheduler {
    /// Returns a new [`Scheduler`].
    pub fn new() -> Self {
        let (sender, receiver) = bounded(1);
        Self {
            scheduler_thread: None,
            scheduler_receiver: receiver,
            scheduler_sender: sender,
        }
    }

    /// Sets up required threads for the provided [`ServiceLocator`] and starts them.
    pub fn run(&mut self, service_locator: Arc<ServiceLocator>) {
        log!(Self, Critical, "Starting scheduler thread.");

        let receiver = self.scheduler_receiver.clone();
        self.scheduler_thread = Some(thread::spawn(move || {
            while let Ok(request) = receiver.recv() {
                log!(Self, Critical, "Update...");
                service_locator.iter().for_each(|(_service_type, service)| {
                    service.update(service_locator.clone());
                });

                request.done.send(()).unwrap();
            }
        }));
    }
}

/// A struct used to send a message to the scheduler that an update call is ready to be made.
/// Additionally contains a channel used to return a reply back to the sender that the update call has been completed.
pub struct WindowReady {
    pub done: Sender<()>,
}
