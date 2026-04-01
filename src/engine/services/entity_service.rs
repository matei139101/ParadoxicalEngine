use crate::prelude::*;
use crate::resources::events::entity_events::CreateEntityEvent;

pub struct EntityService {
    repositories: Arc<Repositories>,
    event_bus_ptr: Arc<EventBus>,
    async_sender: Sender<Box<dyn Any + Send + Sync>>,
}

impl EntityService {
    pub fn new(
        repositories: Arc<Repositories>,
        event_bus_ptr: Arc<EventBus>,
        async_sender: Sender<Box<dyn Any + Send + Sync>>,
    ) -> Arc<Self> {
        let entity_service = Arc::new(EntityService {
            repositories,
            event_bus_ptr: event_bus_ptr.clone(),
            async_sender,
        });

        EntityService::observe_events(entity_service.clone());

        entity_service
    }

    pub fn observe_events(self_ptr: Arc<EntityService>) {
        let bus_arc = {
            self_ptr.event_bus_ptr.clone()
        };

        bus_arc
            .clone()
            .observe::<CreateEntityEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<CreateEntityEvent>() {
                    self_ptr.create_entity(&event.entity);
                }
            }));
    }

    fn create_entity(&self, entity: &Box<dyn Entity>) {
        entity.load(
            self.repositories.get_entity_repository(),
            self.async_sender.clone(),
        );
    }

    pub fn update(&self) {
        if let Some(update_functions) = self.repositories.get_entity_repository().get_update_functions() {
            for (_index, function) in update_functions {
                function(self.repositories.clone())
            }
        }

        self.repositories
            .get_input_repository()
            .set_axis("CAMERAY", 0f64);
        self.repositories
            .get_input_repository()
            .set_axis("CAMERAX", 0f64);

    }
}
