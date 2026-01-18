use std::sync::{Arc, Mutex};

use tokio::sync::oneshot;

use crate::engine::{
    utils::structs::transform::Transform,
    vulkan::{structs::vertex::Vertex, vulkan_container::VulkanContainer},
};

pub struct CreateVulkanInstanceEvent {
    pub vulkan_container: Arc<Mutex<VulkanContainer>>,
}

pub struct VulkanDrawEvent {
    pub player_id: i16,
    pub confirmation_sender: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

/*
pub struct ViewportResizeInfo {
    pub viewport_information: ViewportInfo,
}
*/

pub struct VulkanCreateObjectEvent {
    pub object_id: usize,
    pub vertices: Vec<Vertex>,
    pub object_transform: Transform,
    pub texture_path: String,
}

/*
pub struct VulkanDeleteObjectEvent {
    pub object_id: usize,
}
*/
