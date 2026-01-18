use crate::engine::utils::structs::entity::Entity;

pub struct CreateEntityEvent {
    pub entity: Box<dyn Entity>,
}

pub struct Update {}

/*
pub struct DeleteEntityEvent {
    pub entity_id: usize,
}
*/
