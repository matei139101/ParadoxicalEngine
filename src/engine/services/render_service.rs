use crate::prelude::*;

/// Handles all rendering related tasks.
///
/// This takes care of making the required calls to graphicsAPIs for different rendering processes.
pub struct RenderService {
    graphics_api: Arc<RwLock<dyn GraphicsAPI>>,
}

impl RenderService {
    pub fn new() -> Self {
        Self {
            graphics_api: Arc::new(RwLock::new(Vulkan::new())),
        }
    }
}

impl Service for RenderService {
    fn update(&self) {
        self.graphics_api.read().unwrap().render_frame();
    }

    fn get_data(&self) {}
}

/// A trait a graphicsAPI must implement to be usable by the [`RenderService`]. 
///
/// This is to allow for easy addition of new APIs in the future by making graphicsAPI calls more
/// generic.
trait GraphicsAPI: Any + Send + Sync {
    fn render_frame(&self);
}

/// Defines a struct containing all vulkan functionalities for different processes.
struct Vulkan {}

impl Vulkan {
    pub fn new() -> Self {
        Self {  }
    }
}

impl GraphicsAPI for Vulkan {
    fn render_frame(&self) {
        todo!();
    }
}
