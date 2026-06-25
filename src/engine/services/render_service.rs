use crate::prelude::*;

pub struct RenderService {
    graphics_api: Arc<RwLock<GraphicsAPI>>
}

impl RenderService {
    pub fn new() -> Self {
        Self { graphics_api: Arc::new(RwLock::new(GraphicsAPI::Vulkan())) }
    }
}

impl Service for RenderService {
    fn update(&self) {
        
    }

    fn get_data(&self) {
        todo!()
    }
}

enum GraphicsAPI {
    Vulkan(),
    DirectX12(),
}
