use std::thread::JoinHandle;

use crate::prelude::*;

pub struct Synchronizer {
    services: Arc<Services>,
}

impl Synchronizer {
    pub fn new(services: Arc<Services>) -> Synchronizer {
        Synchronizer { services }
    }

    pub fn start(&self) -> JoinHandle<()> {
        let services = Arc::clone(&self.services);

        thread::spawn(move || loop {
            services.get_vulkan_service().update();
            services.get_entity_service().update();
        })
    }
}
