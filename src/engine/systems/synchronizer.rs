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
            if let Ok(mut vulkan_service) = services.get_vulkan_service().lock() {
                vulkan_service.update();
            } else {
                log!(
                    Self,
                    Critical,
                    "Couldn't lock vulkan service for updating..."
                );
            }

            if let Ok(mut entity_service) = services.get_entity_service().lock() {
                entity_service.update();
            } else {
                log!(
                    Self,
                    Critical,
                    "Couldn't lock entity service for updating..."
                );
            }
        })
    }
}
