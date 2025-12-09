use crate::engine::services::entity_service::entities::entity::Entity;

pub struct CreateEntityEvent {
    pub entity: Box<dyn Entity>,
}

/*
pub struct DeleteEntityEvent {
    pub entity_id: usize,
}
*/
