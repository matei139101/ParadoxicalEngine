use crate::engine::utils::structs::entity::Entity;

pub struct CreateEntityEvent {
    pub entity: Box<dyn Entity>,
}
