use crate::engine::vulkan::structs::vertex::Vertex;
use crate::engine::vulkan::vulkan_container::{VulkanContainer};
use crate::resources::events::vulkan_events::{VulkanCreateObjectEvent};
use crate::{prelude::*};

pub struct VulkanService {
    repositories: Arc<Repositories>,
    vulkan_container: Arc<RwLock<Option<VulkanContainer>>>,
    event_bus_ptr: Arc<EventBus>,
}

impl VulkanService {
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

    pub fn observe_events(self_ptr: Arc<VulkanService>) {
        let bus_arc = self_ptr.event_bus_ptr.clone();

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<VulkanCreateObjectEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<VulkanCreateObjectEvent>() {
                    self_ptr_clone.create_vulkan_object(
                        &event.object_id,
                        &event.vertices,
                        &event.object_transform,
                        &event.texture_path,
                    );
                }
            }));
    }

    pub fn create_vulkan_container(&self, new_container: VulkanContainer) {
        if let Ok(mut vulkan_container) = self.vulkan_container.write() {
            *vulkan_container = Some(new_container);
        } else {
            log!(Self, Critical, "Failed to writelock vulkan_container...");
        }
    }

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

        crate::DEBUGGER.new_frame();
    }

    fn create_vulkan_object(
        &self,
        object_id: &usize,
        vertices: &Vec<Vertex>,
        object_transform: &Transform,
        texture_path: &String,
    ) {
        if let Ok(mut vulkan_container) = self.vulkan_container.write() {
            if let Some(vulkan_container) = vulkan_container.as_mut() {
                vulkan_container.create_vulkan_object(object_id, vertices, object_transform, texture_path);
            } else {
                log!(Self, Critical, "VulkanContainer not found...");
            }
        } else {
            log!(Self, Critical, "Failed to writelock VulkanContainer...");
        }
    }

    pub fn update(&self) {
        self.draw_frame(1);
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
