use crate::prelude::*;

/// Holds all engine services.
///
/// This is to ensure the engine has one central spot for storing services which can later be used
/// for dependendcy injection or calling other service functions across services.
pub struct ServiceLocator {
    services: HashMap<TypeId, Arc<dyn Service>>,
}

impl ServiceLocator {
    /// Returns a new [`ServiceLocator`] with empty services.
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Adds a service to the [`ServiceLocator`] to be used.
    pub fn add_service(&mut self, service: Arc<dyn Service>) {
        let type_id = service.as_ref().type_id();
        self.services.insert(type_id, service);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &Arc<dyn Service>)> {
        self.services.iter()
    }
}

/// A trait which needs to be implemented for a [`Service`] to be usable by the [`ServiceLocator`]
/// and the rest of the engine.
pub trait Service: Any + Send + Sync {
    fn update(&self);
    fn get_data(&self);
}
