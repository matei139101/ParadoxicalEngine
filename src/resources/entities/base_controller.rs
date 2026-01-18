use crate::engine::utils::structs::{entity::Entity, transform::Transform};

pub struct BaseController {
    name: String,
    transform: Transform,
    player_id: i16,
}

impl BaseController {
    pub fn new(name: String, transform: Transform, player_id: i16) -> BaseController {
        BaseController {
            name,
            transform,
            player_id,
        }
    }
}

impl Entity for BaseController {
    fn load(
        &self,
        repository: std::sync::Arc<
            crate::engine::repositories::entity_repository::EntityRepository,
        >,
        _async_sender: tokio::sync::mpsc::UnboundedSender<Box<dyn std::any::Any + Send + Sync>>,
    ) {
        let id = repository.get_id();
        repository.add_entity(id, self.name.clone());
        repository.add_transform(id, self.transform.clone());
        repository.add_controller(id, self.player_id);
    }
}
