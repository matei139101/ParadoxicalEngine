use crate::prelude::*;
use crate::resources::events::entity_events::CreateEntityEvent;

/**
 * A service responsible for handling all entity related events such as creating & deleting entities, handing
 * entity update calls, and handling event related events.
 *
 * Makes use of an [`EntityRepository`] to store entities and their various components.
 */
pub struct EntityService {
    repositories: Arc<Repositories>,
    event_bus_ptr: Arc<EventBus>,
    async_sender: Sender<Box<dyn Any + Send + Sync>>,
}

impl EntityService {
    /**
     * Returns a new entity service which listens to events provided by the given event bus, sends
     * events when necessary through the provided channel, and stores/reads data to and from the
     * provided repositories.
     */
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

    /**
     * Assigns handlers for different events listened to by this service.
     */
    pub fn observe_events(self_ptr: Arc<EntityService>) {
        let bus_arc = { self_ptr.event_bus_ptr.clone() };

        bus_arc
            .clone()
            .observe::<CreateEntityEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<CreateEntityEvent>() {
                    self_ptr.create_entity(&event.entity);
                }
            }));
    }

    /**
     * Calls the dedicated load method for the given entity.
     */
    fn create_entity(&self, entity: &Box<dyn Entity>) {
        entity.load(
            self.repositories.get_entity_repository(),
            self.async_sender.clone(),
        );
    }

    /**
     * Calls each update function stored in the entity repository. Currently also manually resets
     * the camera control axes, this is to ensure the axes only reset after the entities have
     * updated, but is only a temporary workaround until a better solution is found.
     */
    pub fn update(&self, services: &Services) {
        if let Some(update_functions) = self
            .repositories
            .get_entity_repository()
            .get_update_functions()
        {
            for (_index, function) in update_functions {
                function(services, &self.repositories)
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
