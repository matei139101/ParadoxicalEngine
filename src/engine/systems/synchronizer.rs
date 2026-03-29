use std::thread::{sleep};

use crate::prelude::{service::Service, *};

pub struct Synchronizer {
    services: Arc<Services>,
}

impl Synchronizer {
    pub fn new(services: Arc<Services>) -> Synchronizer {
        Synchronizer { services }
    }

    pub fn start(&self) {
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
}
