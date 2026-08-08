use crate::prelude::*;

/// Holds all engine services.
///
/// This is to ensure the engine has one central spot for storing services which can later be used
/// for dependendcy injection or calling other service functions across services.
pub struct ServiceLocator {
    render_service: Arc<RenderService>,
    debug_service: Arc<DebugService>,
}

impl ServiceLocator {
    /// Returns a new [`ServiceLocator`] with empty services.
    pub fn new(render_service: RenderService, debug_service: DebugService) -> Self {
        Self {
            render_service: Arc::new(render_service),
            debug_service: Arc::new(debug_service),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (TypeId, Arc<dyn Service>)> {
        [
            (
                self.render_service.as_ref().type_id(),
                self.render_service.clone() as Arc<dyn Service>,
            ),
            (
                self.debug_service.as_ref().type_id(),
                self.debug_service.clone() as Arc<dyn Service>,
            ),
        ]
        .into_iter()
    }

    pub fn get_render_service(&self) -> Arc<RenderService> {
        self.render_service.clone()
    }

    pub fn get_debug_service(&self) -> Arc<DebugService> {
        self.debug_service.clone()
    }
}

/// A trait which needs to be implemented for a [`Service`] to be usable by the [`ServiceLocator`]
/// and the rest of the engine.
pub trait Service: Any + Send + Sync {
    fn update(&self, service_locator: Arc<ServiceLocator>);
    fn get_data(&self);
}
