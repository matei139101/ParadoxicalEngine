use tokio::sync::mpsc::UnboundedSender;

use crate::engine::{
    event_bus::event_bus::EventBus,
    repositories::entity_repository::EntityRepository,
    services::{
        entity_service::{entities::entity::Entity, entity_events::CreateEntityEvent},
        vulkan_service::vulkan_events::VulkanCreateObjectEvent,
    },
};
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

pub struct EntityService {
    entity_repository: EntityRepository,
    event_bus_ptr: Arc<EventBus>,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
}

impl EntityService {
    pub fn new(
        event_bus_ptr: Arc<EventBus>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    ) -> Arc<Mutex<Self>> {
        let entity_service = Arc::new(Mutex::new(EntityService {
            entity_repository: EntityRepository::new(),
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
                        temp_self.create_entity(event.entity.clone());
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

    fn create_entity(&mut self, entity: Box<dyn Entity>) {
        let object_transform = entity.get_transform().clone();
        let entity_id = self.entity_repository.add_entity(entity.clone());

        if let Some(render_entity) = entity.as_rendered_model() {
            let vulkan_event = VulkanCreateObjectEvent {
                object_id: *entity_id,
                vertices: render_entity.get_model().get_model().to_vec(),
                object_transform,
                texture_path: render_entity.get_texture(),
            };

            let _ = self.async_sender.send(Box::new(vulkan_event));
        }
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
