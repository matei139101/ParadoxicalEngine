use glam::vec3;

use crate::engine::{
    components::entity_component::{
        entities::entity::Entity, entity_traits::player_controller::PlayerController,
    },
    utils::structs::transform::Transform,
};

#[derive(Clone)]
pub struct PlayerEntity {
    transform: Transform,
}

impl PlayerEntity {
    pub fn new() -> Self {
        PlayerEntity {
            transform: Transform::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0)),
        }
    }
}

impl Entity for PlayerEntity {
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
}

impl PlayerController for PlayerEntity {
    fn handle_movement(&mut self) {
        todo!()
    }
}
