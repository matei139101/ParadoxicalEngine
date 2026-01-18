use tokio::sync::mpsc::UnboundedSender;

use crate::{
    engine::{
        event_bus::event_bus::EventBus, services::entity_service::entity_events::CreateEntityEvent,
        utils::structs::entity::Entity,
    },
    Repositories,
};
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

pub struct EntityService {
    repositories: Arc<Repositories>,
    event_bus_ptr: Arc<EventBus>,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
}

impl EntityService {
    pub fn new(
        repositories: Arc<Repositories>,
        event_bus_ptr: Arc<EventBus>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    ) -> Arc<Mutex<Self>> {
        let entity_service = Arc::new(Mutex::new(EntityService {
            repositories,
            event_bus_ptr: event_bus_ptr.clone(),
            async_sender,
        }));

        EntityService::observe_events(entity_service.clone());

        entity_service
    }

    pub fn observe_events(self_ptr: Arc<Mutex<EntityService>>) {
        let bus_arc = {
            let this = self_ptr.lock().unwrap();
            this.event_bus_ptr.clone()
        };

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<CreateEntityEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<CreateEntityEvent>() {
                    if let Ok(mut temp_self) = self_ptr_clone.lock() {
                        temp_self.create_entity(&event.entity);
                    }
                }
            }));

        /*
        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<DeleteEntityEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<DeleteEntityEvent>() {
                    if let Ok(mut temp_self) = self_ptr_clone.lock() {
                        temp_self.delete_entity(&event.entity_id);
                    }
                }
            }));
        */
    }

    fn create_entity(&mut self, entity: &Box<dyn Entity>) {
        entity.load(
            self.repositories.get_entity_repository(),
            self.async_sender.clone(),
        );
    }

    /*
    fn delete_entity(&mut self, entity_id: &usize) {
        self.entity_repository.remove_entity(entity_id);

        let vulkan_delete_event = VulkanDeleteObjectEvent {
            object_id: *entity_id,
        };

        let _ = self.async_sender.send(Box::new(vulkan_delete_event));
    }
    */
}
