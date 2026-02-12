use glam::Vec2;

use crate::{
    engine::vulkan::structs::vertex::Vertex, prelude::*,
    resources::events::vulkan_events::VulkanCreateObjectEvent,
};

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
                    Vec3::new(-0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ), // bottom-left
                Vertex::new(
                    Vec3::new(0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ), // top-right
                Vertex::new(
                    Vec3::new(0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ), // bottom-right
                Vertex::new(
                    Vec3::new(-0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ), // bottom-left
                Vertex::new(
                    Vec3::new(-0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ), // top-left
                Vertex::new(
                    Vec3::new(0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ), // top-right
                // Back face (-Z)
                Vertex::new(
                    Vec3::new(0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                // Left face (-X)
                Vertex::new(
                    Vec3::new(-0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                // Right face (+X)
                Vertex::new(
                    Vec3::new(0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                // Top face (+Y)
                Vertex::new(
                    Vec3::new(-0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, 0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, 0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                // Bottom face (-Y)
                Vertex::new(
                    Vec3::new(-0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, -0.5, -0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(0.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(-0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 0.0),
                ),
                Vertex::new(
                    Vec3::new(0.5, -0.5, 0.5),
                    Vec3::new(255.0, 255.0, 255.0),
                    Vec2::new(1.0, 1.0),
                ),
            ]),
            texture_path: "src/engine/vulkan/base_resources/default_texture.png".to_string(),
        }
    }
}

impl Entity for BaseCube {
    fn load(
        &self,
        repository: Arc<EntityRepository>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    ) {
        if let Some(id) = repository.get_id() {
            repository.add_entity(id, self.name.clone());
            repository.set_transform(id, self.transform.clone());

            let vulkan_event = VulkanCreateObjectEvent {
                object_id: id,
                vertices: self.model.get_model().to_vec(),
                object_transform: self.transform.clone(),
                texture_path: self.texture_path.clone(),
            };

            let _ = async_sender.send(Box::new(vulkan_event));
        } else {
            log!(Self, Critical, "Failed to get id for entity...");
        }
    }
}
