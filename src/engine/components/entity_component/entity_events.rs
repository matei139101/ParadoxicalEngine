use crate::engine::components::entity_component::entities::entity::Entity;

pub struct CreateEntityEvent {
    pub entity: Box<dyn Entity>,
}

/*
pub struct DeleteEntityEvent {
    pub entity_id: usize,
}
*/
