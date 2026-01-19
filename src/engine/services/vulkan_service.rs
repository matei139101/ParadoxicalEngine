use crate::{
    engine::{
        event_bus::event_bus::EventBus,
        utils::structs::{repositories::Repositories, transform::Transform},
        vulkan::{structs::vertex::Vertex, vulkan_container::VulkanContainer},
    },
    resources::events::vulkan_events::{
        CreateVulkanInstanceEvent, VulkanCreateObjectEvent, VulkanDrawEvent,
    },
    widget,
};
use std::{
    sync::{Arc, Mutex, RwLock},
    time::{Duration, Instant},
};

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
            .observe::<VulkanDrawEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<VulkanDrawEvent>() {
                    if let Ok(mut vulkan) = self_ptr_clone.lock() {
                        vulkan.draw_frame(event.player_id);
                        let _ = event
                            .confirmation_sender
                            .lock()
                            .unwrap()
                            .take()
                            .unwrap()
                            .send(());
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
        let camera_transform = self
            .repositories
            .get_entity_repository()
            .get_camera_transform(player_id);
        self.vulkan_container
            .as_mut()
            .unwrap()
            .lock()
            .unwrap()
            .draw_frame(camera_transform);

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

    /*:w

    fn resize_viewport(&mut self, event_info: &ViewportResizeInfo) {
        self.vulkan_container
            .resize_viewport(&event_info.viewport_information);
    }


    fn delete_vulkan_object(&mut self, object_id: &usize) {
        self.vulkan_container.delete_vulkan_object(*object_id);
    }
    */
}
