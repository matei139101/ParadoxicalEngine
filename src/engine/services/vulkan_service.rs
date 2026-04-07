use crate::engine::services::service::Service;
use crate::engine::vulkan::vulkan_container::{VulkanContainer};
use crate::resources::events::vulkan_events::{VulkanCreateObjectEvent};
use crate::{prelude::*};

/// Defines the vulkan service.
///
/// The vulkan service compresses the vast amount of calls made
/// to the [`VulkanContainer`] and exposes them to events and the engine itself for use.
pub struct VulkanService {
    repositories: Arc<Repositories>,
    vulkan_container: Arc<RwLock<Option<VulkanContainer>>>,
    event_bus_ptr: Arc<EventBus>,
}

impl VulkanService {
    /// Returns an [`Arc`] pointer to a new vulkan service which listens to an [`EventBus`] and
    /// reads/stores data to and from the given repositories.
    pub fn new(
        repositories: Arc<Repositories>,
        event_bus_ptr: Arc<EventBus>,
    ) -> Arc<VulkanService> {
        let vulkan_service = Arc::new(VulkanService {
            repositories,
            vulkan_container: Default::default(),
            event_bus_ptr,
        });

        VulkanService::observe_events(vulkan_service.clone());

        vulkan_service
    }

    /// Assigns handlers to ovserved events on the [`EventBus`]. Not meant for manual use.
    pub fn observe_events(self_ptr: Arc<VulkanService>) {
        let bus_arc = self_ptr.event_bus_ptr.clone();

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<VulkanCreateObjectEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<VulkanCreateObjectEvent>() {
                    self_ptr_clone.create_vulkan_object(
                        &event.object_id,
                        &event.mesh,
                        &event.object_transform,
                    );
                }
            }));
    }

    /// Stores the [`VulkanContainer`] for further use when a new one is created by the window.
    /// Inefficient and slow, likely to be reworked in the near future.
    pub fn create_vulkan_container(&self, new_container: VulkanContainer) {
        if let Ok(mut vulkan_container) = self.vulkan_container.write() {
            *vulkan_container = Some(new_container);
        } else {
            log!(Self, Critical, "Failed to writelock vulkan_container...");
        }
    }

    /// Draws a frame to the window from the perspective of the given player id.
    fn draw_frame(&self, player_id: i16) {
        if let Some(camera_transform) = self
            .repositories
            .get_entity_repository()
            .get_camera_transform(player_id)
        {
            if let Ok(mut vulkan_container) = self.vulkan_container.write() {
                if let Some(vulkan_container) = vulkan_container.as_mut() {
                    vulkan_container.draw_frame(camera_transform);
                } else {
                    log!(Self, High, "VulkanContainer not found...");
                }
            } else {
                log!(Self, High, "Failed to writelock VulkanContainer...");
            }
        } else {
            log!(Self, High, "Failed to get camera transform...");
        }

        crate::LOGGER.new_frame();
    }

    /// Creates a vulkan object usable by vulkan during rendering from a given mesh.
    fn create_vulkan_object(
        &self,
        object_id: &usize,
        mesh: &String,
        object_transform: &Transform,
    ) {
        if let Ok(mut vulkan_container) = self.vulkan_container.write() {
            if let Some(vulkan_container) = vulkan_container.as_mut() {
                let mesh = Mesh::from_gltf(mesh)[0].clone();
                vulkan_container.create_vulkan_object(object_id, mesh, object_transform);
            } else {
                log!(Self, Critical, "VulkanContainer not found...");
            }
        } else {
            log!(Self, Critical, "Failed to writelock VulkanContainer...");
        }
    }

    /*

    fn resize_viewport(&mut self, event_info: &ViewportResizeInfo) {
        self.vulkan_container
            .resize_viewport(&event_info.viewport_information);
    }


    fn delete_vulkan_object(&mut self, object_id: &usize) {
        self.vulkan_container.delete_vulkan_object(*object_id);
    }
    */
}

impl Service for VulkanService {
    fn update(&self) {
        self.draw_frame(1);
    }

    fn is_ready(&self) -> bool {
        if let Ok(vulkan_container) = self. vulkan_container.read() {
            vulkan_container.is_some()
        } else {
            false
        }
    }
}
