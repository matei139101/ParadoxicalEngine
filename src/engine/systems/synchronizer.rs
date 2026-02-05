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
        log!(Self, High, "KLANKERRRRR");

        thread::spawn(move || {
            loop {
                log!(Self, High, "HEEEEEEELP!");
                if let Ok(mut vulkan_service) = services.get_vulkan_service().lock() {
                    vulkan_service.update();
                } else {
                    panic!("Couldn't lock vulkan service for updating...");
                }
                log!(Self, High, "I dont know");
            }
        })
    }
}
