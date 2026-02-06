use crate::engine::vulkan::structs::vertex::Vertex;
use crate::engine::vulkan::vulkan_container::VulkanContainer;
use crate::resources::events::vulkan_events::{CreateVulkanInstanceEvent, VulkanCreateObjectEvent};
use crate::{prelude::*, widget};

pub struct VulkanService {
    repositories: Arc<Repositories>,
    vulkan_container: Option<Arc<Mutex<VulkanContainer>>>,
    event_bus_ptr: Arc<EventBus>,
    last_framecheck: Instant,
    frame_count: u32,

    fps: Arc<RwLock<f32>>,
}

impl VulkanService {
    pub fn new(
        repositories: Arc<Repositories>,
        event_bus_ptr: Arc<EventBus>,
    ) -> Arc<Mutex<VulkanService>> {
        let vulkan_service = Arc::new(Mutex::new(VulkanService {
            repositories,
            vulkan_container: Default::default(),
            event_bus_ptr,
            last_framecheck: Instant::now(),
            frame_count: 0,
            fps: Arc::new(RwLock::new(0.0)),
        }));

        VulkanService::observe_events(vulkan_service.clone());

        widget!(
            "FPS".to_string(),
            vulkan_service.lock().unwrap().fps.clone()
        );

        vulkan_service
    }

    pub fn observe_events(self_ptr: Arc<Mutex<VulkanService>>) {
        let bus_arc = {
            let this = self_ptr.lock().unwrap();
            this.event_bus_ptr.clone()
        };

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<CreateVulkanInstanceEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<CreateVulkanInstanceEvent>() {
                    if let Ok(mut vulkan) = self_ptr_clone.lock() {
                        vulkan.create_vulkan_container(event.vulkan_container.clone());
                    }
                }
            }));

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<VulkanCreateObjectEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<VulkanCreateObjectEvent>() {
                    if let Ok(mut vulkan) = self_ptr_clone.lock() {
                        vulkan.create_vulkan_object(
                            &event.object_id,
                            &event.vertices,
                            &event.object_transform,
                            &event.texture_path,
                        );
                    }
                }
            }));
    }

    fn create_vulkan_container(&mut self, vulkan_container: Arc<Mutex<VulkanContainer>>) {
        self.vulkan_container = Some(vulkan_container);
    }

    fn draw_frame(&mut self, player_id: i16) {
        if let Some(camera_transform) = self
            .repositories
            .get_entity_repository()
            .get_camera_transform(player_id)
        {
            if let Some(vulkan_container) = self.vulkan_container.as_mut() {
                if let Ok(mut vulkan_container) = vulkan_container.lock() {
                    vulkan_container.draw_frame(camera_transform);
                } else {
                    log!(Self, High, "Failed to lock vulkan container...");
                }
            } else {
                log!(Self, High, "Failed to unwrap vulkan container...");
            }
        } else {
            log!(Self, High, "Failed to get camera transform...");
        }

        self.frame_count += 1;
        let elapsed = self.last_framecheck.elapsed();

        if elapsed >= Duration::from_secs(1) {
            *self.fps.write().unwrap() = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_framecheck = Instant::now();
        }
    }

    fn create_vulkan_object(
        &mut self,
        object_id: &usize,
        vertices: &Vec<Vertex>,
        object_transform: &Transform,
        texture_path: &String,
    ) {
        self.vulkan_container
            .as_mut()
            .unwrap()
            .lock()
            .unwrap()
            .create_vulkan_object(object_id, vertices, object_transform, texture_path);
    }

    pub fn update(&mut self) {
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
