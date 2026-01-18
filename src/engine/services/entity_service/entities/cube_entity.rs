use glam::{vec2, vec3};

use crate::engine::{
    services::entity_service::{
        entities::entity::Entity, entity_traits::rendered_model::RenderedModel,
    },
    utils::structs::{model::Model, transform::Transform},
    vulkan::structs::vertex::Vertex,
};

#[derive(Clone)]
pub struct CubeEntity {
    transform: Transform,
    model: Model,
    texture_path: String,
}

impl CubeEntity {
    pub fn new(transform: Transform) -> Self {
        CubeEntity {
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

impl Entity for CubeEntity {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_rendered_model(&self) -> Option<&dyn RenderedModel> {
        Some(self)
    }
}

impl RenderedModel for CubeEntity {
    fn get_model(&self) -> &Model {
        &self.model
    }

    fn get_texture(&self) -> String {
        self.texture_path.clone()
    }
}
