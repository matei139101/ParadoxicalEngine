use crate::engine::utils::structs::{entity::Entity, transform::Transform};
use crate::prelude::*;

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

    fn update(repositories: Arc<Repositories>) {
        if let Some(player_entity_id) = 
            repositories
            .get_entity_repository()
            .get_player_controller(1)
        {
            if let Some(mut transform) =
                repositories
                .get_entity_repository()
                .get_transform(player_entity_id)
            {
                if let Some(input_y) = repositories.get_input_repository().get_axis("CAMERAY")
                {
                    transform.offset_y_rotation(input_y * 0.0025);
                } else {
                    log!(Self, Critical, "Failed to update controller transform...");
                }

                if let Some(input_x) = repositories.get_input_repository().get_axis("CAMERAX")
                {
                    transform.offset_x_rotation(input_x * -0.0025);
                } else {
                    log!(Self, Critical, "Failed to update controller transform...");
                }

                if let Some(input_forward) = repositories.get_input_repository().get_axis("FORWARD")
                {
                    transform.position = transform.get_position() + transform.forward() * -input_forward as f32;
                } else {
                    log!(Self, Critical, "Failed to update controller transform...");
                }

                if let Some(input_right) = repositories.get_input_repository().get_axis("RIGHT")
                {
                    transform.position = transform.get_position() + transform.right() * -input_right as f32;
                } else {
                    log!(Self, Critical, "Failed to update controller transform...");
                }
                
                repositories
                    .get_entity_repository()
                    .set_transform(player_entity_id, transform);

            } else {
                log!(Self, Critical, "Failed to get player entity transform...");
            }
        } else {
            log!(Self, Critical, "Failed to get player entity Id...");
        }

    }

}

impl Entity for BaseController {
    fn load(
        &self,
        repository: std::sync::Arc<
            crate::engine::repositories::entity_repository::EntityRepository,
        >,
        _async_sender: Sender<Box<dyn std::any::Any + Send + Sync>>,
    ) {
        if let Some(id) = repository.get_id() {
            repository.add_entity(id, self.name.clone());
            repository.set_transform(id, self.transform.clone());
            repository.add_controller(id, self.player_id);
            repository.add_update_function(id, BaseController::update);

        } else {
            log!(Self, Critical, "Failed to get id for entity...");
        }
    }
}
