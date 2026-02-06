use crate::prelude::*;
use crate::resources::events::entity_events::CreateEntityEvent;

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
    }

    fn create_entity(&mut self, entity: &Box<dyn Entity>) {
        entity.load(
            self.repositories.get_entity_repository(),
            self.async_sender.clone(),
        );
    }

    pub fn update(&mut self) {
        if let Some(player_entity_id) = self
            .repositories
            .get_entity_repository()
            .get_player_controller(1)
        {
            if let Some(mut transform) = self
                .repositories
                .get_entity_repository()
                .get_transform(player_entity_id)
            {
                if let Some(input_y) = self.repositories.get_input_repository().get_axis("CAMERAY")
                {
                    transform.offset_y_rotation(input_y * 0.0025);
                } else {
                    log!(Self, Critical, "Failed to update camera transform...");
                }

                if let Some(input_x) = self.repositories.get_input_repository().get_axis("CAMERAX")
                {
                    transform.offset_x_rotation(input_x * -0.0025);
                    self.repositories
                        .get_entity_repository()
                        .set_transform(player_entity_id, transform);
                } else {
                    log!(Self, Critical, "Failed to update camera transform...");
                }

                self.repositories
                    .get_input_repository()
                    .set_axis("CAMERAY", 0f64);
                self.repositories
                    .get_input_repository()
                    .set_axis("CAMERAX", 0f64);
            } else {
                log!(Self, Critical, "Failed to get player entity transform...");
            }
        } else {
            log!(Self, Critical, "Failed to get player entity Id...");
        }
    }
}
