use crate::prelude::*;

pub struct Services {
    input_service: Arc<InputService>,
    entity_service: Arc<EntityService>,
    vulkan_service: Arc<VulkanService>,
}

impl Services {
    pub fn new(
        repositories: Arc<Repositories>,
        event_bus_ptr: Arc<EventBus>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    ) -> Services {
        Services { 
            input_service: InputService::new(repositories.clone()), 
            entity_service: EntityService::new(repositories.clone(), event_bus_ptr.clone(), async_sender), 
            vulkan_service: VulkanService::new(repositories.clone(), event_bus_ptr.clone()),
        }
    }

    pub fn get_input_service(&self) -> Arc<InputService> {
        self.input_service.clone()
    }

    pub fn get_entity_service(&self) -> Arc<EntityService> {
        self.entity_service.clone()
    }

    pub fn get_vulkan_service(&self) -> Arc<VulkanService> {
        self.vulkan_service.clone()
    }
}
