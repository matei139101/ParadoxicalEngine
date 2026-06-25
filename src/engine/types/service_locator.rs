use crate::prelude::*;

/// Holds all engine services.
///
/// This is to ensure the engine has one central spot for storing services which can later be used
/// for dependendcy injection or calling other service functions across services.
pub struct ServiceLocator {
    services: HashMap::<ServiceType, Arc::<dyn Service>>
}

impl ServiceLocator {
    /// Returns a new [`ServiceLocator`] with empty services.
    pub fn new() -> Self {
        Self { services: HashMap::new() }
    }

    /// Adds a service to the [`ServiceLocator`] to be used.
    pub fn add_service(&mut self, service_type: ServiceType, service: Arc<dyn Service>) {
        self.services.insert(service_type, service);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ServiceType, &Arc<dyn Service>)> {
        self.services.iter()
    }
}

/// A trait which needs to be implemented for a [`Service`] to be usable by the [`ServiceLocator`]
/// and the rest of the engine.
pub trait Service: Send + Sync {
    fn update(&self);
    fn get_data(&self);
}

/// An enum used to give coherency to the service locator.
///
/// Each new [`Service`] type must be added here for consistency to be used by the [`ServiceLocator`] and the rest of the engine in a unified manner.
#[derive(Eq, PartialEq, Hash)]
pub enum ServiceType {
    InputService,
    RenderService,
    EntityService,
}
