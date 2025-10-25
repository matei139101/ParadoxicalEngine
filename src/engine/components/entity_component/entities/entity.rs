use std::any::Any;

use dyn_clone::DynClone;

use crate::engine::{
    components::entity_component::entity_traits::{
        player_controller::PlayerController, rendered_model::RenderedModel,
    },
    utils::structs::transform::Transform,
};

pub trait Entity: Send + Sync + DynClone {
    fn get_transform(&self) -> &Transform;
    fn set_transform(&mut self, transform: Transform);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_player_controller(&self) -> Option<&dyn PlayerController> {
        None
    }

    fn as_rendered_model(&self) -> Option<&dyn RenderedModel> {
        None
    }
}

dyn_clone::clone_trait_object!(Entity);
