use std::thread::{sleep};
use crate::prelude::*;

pub struct Scheduler {
    services: Arc<Services>,
}

impl Scheduler {
    pub fn new(services: Arc<Services>) -> Scheduler {
        Scheduler { services }
    }

    pub fn start(&self) {
        self.make_auxilliary_thread();
        self.make_service_thread();
    }

    fn make_service_thread(&self) {
        log!(Self, Critical, "Starting service thread...");
        let services = Arc::clone(&self.services);

        let _ = thread::Builder::new().name("Synchronizer".to_string()).spawn(move || {
            while !services.get_vulkan_service().is_ready() {
                sleep(Duration::from_millis(10));
            }

            loop {
                services.get_vulkan_service().update();
                services.get_entity_service().update();
            }
        });
    }

    fn make_auxilliary_thread(&self) {
        log!(Self, Critical, "Starting auxilliary thread...");
        let _ = thread::Builder::new().name("Auxilliary".to_string()).spawn(move || {
            loop {
                LOGGER.update();
                DASHBOARD.update();
            }
        });
    }
}
