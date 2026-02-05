use crate::prelude::*;

pub struct Services {
    input_service: Arc<Mutex<InputService>>,
    entity_service: Arc<Mutex<EntityService>>,
    vulkan_service: Arc<Mutex<VulkanService>>,
}

impl Services {
    pub fn new(
        repositories: Arc<Repositories>,
        event_bus_ptr: Arc<EventBus>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
        input_receiver: UnboundedReceiver<DeviceEvent>,
    ) -> Services {
        Services { 
            input_service: InputService::new(repositories.clone(), input_receiver), 
            entity_service: EntityService::new(repositories.clone(), event_bus_ptr.clone(), async_sender), 
            vulkan_service: VulkanService::new(repositories.clone(), event_bus_ptr.clone()),
        }
    }

    pub fn get_input_service(&self) -> Arc<Mutex<InputService>> {
        self.input_service.clone()
    }

    pub fn get_entity_service(&self) -> Arc<Mutex<EntityService>> {
        self.entity_service.clone()
    }

    pub fn get_vulkan_service(&self) -> Arc<Mutex<VulkanService>> {
        self.vulkan_service.clone()
    }
}
