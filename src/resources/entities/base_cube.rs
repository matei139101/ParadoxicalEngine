use std::{any::Any, sync::Arc};

use glam::{vec2, vec3};
use tokio::sync::mpsc::UnboundedSender;

use crate::engine::{repositories::entity_repository::EntityRepository, services::vulkan_service::vulkan_events::VulkanCreateObjectEvent, utils::structs::{entity::Entity, model::Model, transform::Transform}, vulkan::structs::vertex::Vertex};

#[derive(Clone)]
pub struct BaseCube {
    name: String,
    transform: Transform,
    model: Model,
    texture_path: String,
}

impl BaseCube {
    pub fn new(name: String, transform: Transform) -> BaseCube {
        BaseCube { 
            name,
            transform,
            model: Model::new(vec![
                // Front face (+Z)
                Vertex::new(
                    vec3(-0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ), // bottom-left
                Vertex::new(
                    vec3(0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ), // top-right
                Vertex::new(
                    vec3(0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ), // bottom-right
                Vertex::new(
                    vec3(-0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ), // bottom-left
                Vertex::new(
                    vec3(-0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ), // top-left
                Vertex::new(
                    vec3(0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ), // top-right
                // Back face (-Z)
                Vertex::new(
                    vec3(0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 1.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                Vertex::new(
                    vec3(-0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 1.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                // Left face (-X)
                Vertex::new(
                    vec3(-0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                Vertex::new(
                    vec3(-0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                // Right face (+X)
                Vertex::new(
                    vec3(0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                Vertex::new(
                    vec3(0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                // Top face (+Y)
                Vertex::new(
                    vec3(-0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, 0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, 0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                // Bottom face (-Y)
                Vertex::new(
                    vec3(-0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
                Vertex::new(
                    vec3(0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, -0.5, -0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(0.0, 0.0),
                ),
                Vertex::new(
                    vec3(-0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 0.0),
                ),
                Vertex::new(
                    vec3(0.5, -0.5, 0.5),
                    vec3(255.0, 255.0, 255.0),
                    vec2(1.0, 1.0),
                ),
            ]),
            texture_path: "src/engine/vulkan/base_resources/default_texture.png".to_string(),
        }
    }
}

impl Entity for BaseCube {
    fn load(&self, repository: Arc<EntityRepository>, async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>) {
        let id = repository.get_id();
        repository.add_entity(id, self.name.clone());
        repository.add_transform(id, self.transform.clone());

        let vulkan_event = VulkanCreateObjectEvent {
            object_id: id,
            vertices: self.model.get_model().to_vec(),
            object_transform: self.transform.clone(),
            texture_path: self.texture_path.clone(),
        };

        let _ = async_sender.send(Box::new(vulkan_event));
    }
}
