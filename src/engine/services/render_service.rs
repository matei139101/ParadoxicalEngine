use crate::prelude::*;

/// Handles all rendering related tasks.
pub struct RenderService {
    _graphics_api: Arc<RwLock<GraphicsAPI>>,
}

impl RenderService {
    pub fn new() -> Self {
        Self {
            _graphics_api: Arc::new(RwLock::new(GraphicsAPI::Vulkan())),
        }
    }
}

impl Service for RenderService {
    fn update(&self) {}

    fn get_data(&self) {}
}

enum GraphicsAPI {
    Vulkan(),
    _DirectX12(),
}
